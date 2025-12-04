pub fn run_osdr_scheduler(state: AppState) {
    tokio::spawn(async move {
        loop {
            let pool = state.pool.clone();
            let st = state.clone();

            let _ = run_with_lock(&pool, 1002, || async move {
                st.osdr_service().sync(&st).await?;
                Ok(())
            })
            .await;

            tokio::time::sleep(Duration::from_secs(state.every_osdr)).await;
        }
    });
}
