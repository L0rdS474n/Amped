// M1 tests — RefreshModel variants (amped-plugin-api).
//
// Covers AC-M1-7. All inputs are inline strings (deterministic, no I/O).

// ---------------------------------------------------------------------------
// T-API-refresh-variants — happy paths
// ---------------------------------------------------------------------------

#[test]
fn t_api_refresh_interval_30() {
    use amped_plugin_api::widget::RefreshModel;

    // Given: a widget TOML with model="interval", secs=30
    // When:  parsed as RefreshModel
    // Then:  RefreshModel::Interval { secs: 30 }
    let model = RefreshModel::from_toml_table_str(
        r#"model = "interval"
secs = 30"#,
    )
    .expect("interval secs=30 must parse Ok");

    assert!(
        matches!(model, RefreshModel::Interval { secs: 30 }),
        "expected Interval{{secs:30}}, got {:?}",
        model
    );
}

#[test]
fn t_api_refresh_on_demand() {
    use amped_plugin_api::widget::RefreshModel;

    let model = RefreshModel::from_toml_table_str(r#"model = "on_demand""#)
        .expect("on_demand must parse Ok");

    assert!(
        matches!(model, RefreshModel::OnDemand),
        "expected OnDemand, got {:?}",
        model
    );
}

#[test]
fn t_api_refresh_push() {
    use amped_plugin_api::widget::RefreshModel;

    let model = RefreshModel::from_toml_table_str(r#"model = "push""#).expect("push must parse Ok");

    assert!(
        matches!(model, RefreshModel::Push),
        "expected Push, got {:?}",
        model
    );
}

// ---------------------------------------------------------------------------
// T-API-refresh-variants — negative paths
// ---------------------------------------------------------------------------

#[test]
fn t_api_refresh_interval_missing_secs_is_error() {
    use amped_plugin_api::widget::RefreshModel;

    // Given: interval without secs
    // When:  parsed
    // Then:  Err (interval requires secs)
    let err = RefreshModel::from_toml_table_str(r#"model = "interval""#);
    assert!(
        err.is_err(),
        "interval without secs must be Err, got Ok({:?})",
        err.ok()
    );
}

#[test]
fn t_api_refresh_interval_secs_zero_is_error() {
    use amped_plugin_api::widget::{RefreshError, RefreshModel};

    // Given: interval with secs=0
    // When:  parsed
    // Then:  Err(RefreshError::NonPositiveInterval) — zero-second polling is a footgun
    let err = RefreshModel::from_toml_table_str(
        r#"model = "interval"
secs = 0"#,
    )
    .unwrap_err();

    assert!(
        matches!(err, RefreshError::NonPositiveInterval),
        "expected NonPositiveInterval, got {:?}",
        err
    );
}

#[test]
fn t_api_refresh_bogus_model_is_error() {
    use amped_plugin_api::widget::RefreshModel;

    let err = RefreshModel::from_toml_table_str(r#"model = "bogus_unknown""#);
    assert!(err.is_err(), "unknown model variant must be Err");
}
