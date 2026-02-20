#[derive(Debug, Clone, Default)]
pub struct DashboardStats {
    pub sessions_today: i64,
    pub streak: i64,
    pub total_minutes: i64,
}

/// Minutes of activity for each of the last 7 days, index 0 = 6 days ago, index 6 = today.
pub type WeeklyMinutes = [i64; 7];
