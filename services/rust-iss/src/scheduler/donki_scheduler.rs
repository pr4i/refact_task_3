pub fn run_donki_scheduler(state: AppState) {
    tokio::spawn(async move {
        loop {
            let pool = state.pool.clone();
            let st = state.clone();

            // FLR
            let _ = run_with_lock(&pool, 1005, || async move {
                st.space_service().refresh(&st, "flr").await?;
                Ok(())
            })
            .await;

            // CME
            let _ = run_with_lock(&pool, 1006, || async move {
                st.space_service().refresh(&st, "cme").await?;
                Ok(())
            })
            .await;

            tokio::time::sleep(Duration::from_secs(state.every_donki)).await;
        }
    });
}
