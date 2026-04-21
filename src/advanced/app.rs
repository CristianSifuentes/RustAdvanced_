use crate::advanced::async_pipeline::{CancelToken, block_on, run_pipeline};
use crate::advanced::ffi_boundary::checked_view;
use crate::advanced::patterns::{Analyzer, LabError, MeanAnalyzer, SharedNotebook};
use crate::advanced::scientific::{Mean, Meter, Quantity, Second, Vector, report_with};
use crate::telemetry;

pub fn run_demo() -> Result<(), LabError> {
    // Step 13: typed quantities prevent unit confusion at compile time.
    let track_a = Quantity::<f64, Meter>::new(12.0);
    let track_b = Quantity::<f64, Meter>::new(8.0);
    let combined = [track_a, track_b]
        .as_slice()
        .mean()
        .ok_or(LabError::EmptyExperiment)?;

    // Keep `Second` marker used to demonstrate extra phantom unit type.
    let _time_window = Quantity::<f64, Second>::new(2.5);

    // Step 14: const-generic vector math.
    let v1 = Vector::<3>::new([0.2, 0.4, 0.6]);
    let v2 = Vector::<3>::new([2.0, 1.0, 0.5]);
    let dot = v1.dot(&v2);

    // Step 15: HRTB-powered generic reporting.
    let rows = report_with(&["alpha", "beta", "gamma"], |x| format!("label={x}"));

    // Step 16: async pipeline with cancellation-ready token.
    let token = CancelToken::default();
    let squared = block_on(run_pipeline(vec![1.0, 2.0, 3.0, 4.0], token.clone()));
    token.cancel();

    // Step 17: safe wrapper around unsafe pointer view.
    let bytes = b"rust-ffi-boundary";
    let ffi_view = checked_view(bytes);
    let ffi_checksum: u64 = ffi_view.as_slice().iter().map(|b| *b as u64).sum();

    // Step 18: dynamic dispatch + concurrent notebook.
    let analyzer: Box<dyn Analyzer> = Box::new(MeanAnalyzer);
    let score = analyzer.score(&squared);

    let notebook = SharedNotebook::default();
    notebook.push(format!("analyzer={}", analyzer.name()))?;
    notebook.push(format!("mean_distance={:.2}", combined.raw()))?;
    notebook.push(format!("dot={dot:.2}"))?;
    notebook.push(format!("rows={}", rows.len()))?;
    notebook.push(format!("ffi_checksum={ffi_checksum}"))?;
    notebook.push(format!("pipeline_score={score:.2}"))?;

    // Step 19: macro-generated telemetry map.
    let payload = telemetry! {
        "pipeline_count" => squared.len(),
        "notebook_rows" => notebook.rows()?.len(),
        "dot" => format!("{dot:.2}"),
    };

    println!("--- Rust Advanced Scientific Demo ---");
    for row in notebook.rows()? {
        println!("{row}");
    }
    println!("telemetry={payload:?}");

    Ok(())
}
