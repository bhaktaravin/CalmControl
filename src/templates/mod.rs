use crate::models::{
    session::{DashboardStats, WeeklyMinutes},
    user::User,
    video::{CATEGORIES, VideoWithUploader, category_label},
};

// ── Shared CSS (stored as a raw string to avoid brace-escaping inside format!) ─

const CSS: &str = r#"<style>
    :root {
        --calm-dark:  #1b4332;
        --calm-mid:   #2d6a4f;
        --calm-light: #40916c;
        --calm-pale:  #d8f3dc;
    }
    * { box-sizing: border-box; }
    body {
        background: #f0f7f4;
        font-family: 'Segoe UI', system-ui, -apple-system, sans-serif;
        min-height: 100vh;
        margin: 0;
    }
    /* ── Navbar ── */
    .navbar {
        background: linear-gradient(135deg, var(--calm-dark), var(--calm-light));
        box-shadow: 0 2px 12px rgba(0,0,0,0.18);
    }
    .navbar-brand {
        font-weight: 700;
        font-size: 1.4rem;
        letter-spacing: -0.4px;
    }
    .nav-link { color: rgba(255,255,255,0.85) !important; transition: color .2s; }
    .nav-link:hover { color: #fff !important; }
    /* ── Cards ── */
    .card {
        border: none;
        border-radius: 20px;
        box-shadow: 0 4px 30px rgba(0,0,0,0.07);
    }
    /* ── Buttons ── */
    .btn-calm {
        background: linear-gradient(135deg, var(--calm-mid), var(--calm-light));
        border: none;
        color: #fff;
        border-radius: 10px;
        padding: 10px 24px;
        font-weight: 500;
        transition: all .2s;
    }
    .btn-calm:hover {
        background: linear-gradient(135deg, var(--calm-dark), var(--calm-mid));
        color: #fff;
        transform: translateY(-1px);
        box-shadow: 0 4px 14px rgba(45,106,79,.35);
    }
    /* ── Form controls ── */
    .form-control {
        border-radius: 10px;
        border: 1.5px solid #cde8d6;
        padding: 10px 16px;
    }
    .form-control:focus {
        border-color: var(--calm-light);
        box-shadow: 0 0 0 3px rgba(64,145,108,.18);
        outline: none;
    }
    .form-label { font-weight: 500; color: var(--calm-dark); }
    /* ── Utility ── */
    .text-calm  { color: var(--calm-mid)  !important; }
    .bg-calm    { background: var(--calm-pale); }
    /* ── Stat cards ── */
    .stat-card {
        background: linear-gradient(135deg, var(--calm-mid), var(--calm-light));
        color: #fff;
        border-radius: 18px;
        padding: 26px 24px;
    }
    /* ── Profile avatar ── */
    .profile-avatar {
        width: 82px; height: 82px;
        border-radius: 50%;
        background: linear-gradient(135deg, var(--calm-mid), var(--calm-light));
        display: flex; align-items: center; justify-content: center;
        font-size: 2rem; font-weight: 700; color: #fff;
        flex-shrink: 0;
    }
    /* ── Info rows ── */
    .info-row {
        background: #f0f7f4;
        border-radius: 12px;
        padding: 14px 18px;
    }
    /* ── Bar chart ── */
    .bar-wrap {
        display: flex;
        align-items: flex-end;
        gap: 8px;
        height: 90px;
    }
    .bar {
        flex: 1;
        border-radius: 6px 6px 0 0;
        background: linear-gradient(to top, #40916c, #74c69d);
        opacity: .75;
        transition: opacity .2s;
    }
    .bar:hover { opacity: 1; }
    .bar.active-bar {
        background: linear-gradient(to top, #2d6a4f, #52b788);
        opacity: 1;
    }
    /* ── Breathing circle ── */
    .breath-circle {
        width: 200px;
        height: 200px;
        border-radius: 50%;
        background: radial-gradient(circle at 40% 35%, #74c69d, #2d6a4f);
        margin: 2rem auto;
        transition: transform 4s ease-in-out, box-shadow 4s ease-in-out;
        box-shadow: 0 0 40px rgba(82,183,136,0.35);
        display: flex;
        align-items: center;
        justify-content: center;
        color: #fff;
        font-size: 2.5rem;
    }
    .breath-circle.expand {
        transform: scale(1.45);
        box-shadow: 0 0 70px rgba(82,183,136,0.55);
    }
    /* ── Meditation orb ── */
    .meditate-orb {
        width: 200px;
        height: 200px;
        border-radius: 50%;
        background: radial-gradient(circle at 40% 35%, #b39ddb, #512da8);
        margin: 2rem auto;
        animation: meditate-pulse 6s ease-in-out infinite;
        box-shadow: 0 0 50px rgba(123,63,140,0.35);
        display: flex;
        align-items: center;
        justify-content: center;
        color: #fff;
        font-size: 2.5rem;
    }
    @keyframes meditate-pulse {
        0%, 100% { transform: scale(1);    box-shadow: 0 0 40px rgba(123,63,140,0.30); }
        50%       { transform: scale(1.15); box-shadow: 0 0 80px rgba(123,63,140,0.55); }
    }
    /* ── Progress bar ── */
    .calm-progress {
        height: 8px;
        border-radius: 4px;
        background: #d8f3dc;
        overflow: hidden;
        margin: 1rem 0;
    }
    .calm-progress-bar {
        height: 100%;
        border-radius: 4px;
        background: linear-gradient(90deg, #40916c, #74c69d);
        transition: width 1s linear;
    }
    /* ── Mood picker ── */
    .mood-option input { display: none; }
    .mood-option label {
        font-size: 2.2rem;
        cursor: pointer;
        display: inline-block;
        padding: 6px 10px;
        border-radius: 12px;
        transition: transform .15s, background .15s;
        line-height: 1;
    }
    .mood-option input:checked + label,
    .mood-option label:hover {
        background: var(--calm-pale);
        transform: scale(1.25);
    }
    /* ── Session complete banner ── */
    .session-done-banner {
        display: none;
        background: linear-gradient(135deg, #d8f3dc, #b7e4c7);
        border-radius: 16px;
        padding: 20px 28px;
        text-align: center;
        border: 1.5px solid #74c69d;
        margin-top: 1.5rem;
    }
    /* ── Video cards ── */
    .video-card {
        border-radius: 20px;
        overflow: hidden;
        box-shadow: 0 4px 30px rgba(0,0,0,0.08);
        transition: transform .2s, box-shadow .2s;
        background: #fff;
        border: none;
        height: 100%;
    }
    .video-card:hover {
        transform: translateY(-4px);
        box-shadow: 0 10px 40px rgba(0,0,0,0.13);
    }
    .video-thumb {
        width: 100%;
        aspect-ratio: 16/9;
        object-fit: cover;
        background: #e8f5e9;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 3rem;
    }
    .video-thumb img {
        width: 100%;
        height: 100%;
        object-fit: cover;
    }
    .category-badge {
        display: inline-block;
        padding: 3px 12px;
        border-radius: 20px;
        font-size: .78rem;
        font-weight: 600;
        letter-spacing: .3px;
    }
    .cat-breathing     { background:#d8f3dc; color:#1b4332; }
    .cat-meditation    { background:#ede7f6; color:#4a148c; }
    .cat-nutrition     { background:#fff3e0; color:#e65100; }
    .cat-exercise      { background:#e3f2fd; color:#0d47a1; }
    .cat-mental-health { background:#e0f2f1; color:#004d40; }
    .cat-general       { background:#f5f5f5; color:#424242; }
    /* ── Filter pills ── */
    .filter-pill {
        border: 1.5px solid #cde8d6;
        border-radius: 20px;
        padding: 5px 16px;
        font-size: .85rem;
        font-weight: 500;
        cursor: pointer;
        background: #fff;
        color: var(--calm-mid);
        transition: all .15s;
    }
    .filter-pill:hover,
    .filter-pill.active {
        background: var(--calm-mid);
        color: #fff;
        border-color: var(--calm-mid);
    }
    /* ── Video player ── */
    .video-player-wrap {
        border-radius: 16px;
        overflow: hidden;
        background: #000;
        box-shadow: 0 8px 40px rgba(0,0,0,0.18);
    }
    .video-player-wrap video {
        width: 100%;
        display: block;
        max-height: 520px;
    }
</style>"#;

// ── Shared base layout ─────────────────────────────────────────────────────────

fn base_layout(title: &str, content: &str, logged_in: bool) -> String {
    let nav_items = if logged_in {
        r#"<li class="nav-item">
              <a class="nav-link" href="/dashboard">
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 16 16" style="margin-right:4px"><path d="M8 4a.5.5 0 0 1 .5.5V6a.5.5 0 0 1-1 0V4.5A.5.5 0 0 1 8 4zM3.732 5.732a.5.5 0 0 1 .707 0l.915.914a.5.5 0 1 1-.708.708l-.914-.915a.5.5 0 0 1 0-.707zM2 10a.5.5 0 0 1 .5-.5h1.586a.5.5 0 0 1 0 1H2.5A.5.5 0 0 1 2 10zm9.5 0a.5.5 0 0 1 .5-.5h1.5a.5.5 0 0 1 0 1H12a.5.5 0 0 1-.5-.5zm.754-4.246a.389.389 0 0 0-.527-.02L9.650 7.292a.999.999 0 1 0 1.122 1.657l2.244-2.02a.389.389 0 0 0 .069-.527A.5.5 0 0 1 11.5 6.268z"/></svg>
                  Dashboard
              </a>
           </li>
           <li class="nav-item">
              <a class="nav-link" href="/videos">
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 16 16" style="margin-right:4px"><path d="M0 1a1 1 0 0 1 1-1h14a1 1 0 0 1 1 1v14a1 1 0 0 1-1 1H1a1 1 0 0 1-1-1V1zm4 0v6h6V1H4zm6 8H4v6h6V9zm-6-1h2v1H4V8zm0-3h2v1H4V5zm6 3h2v1h-2V8zm0-3h2v1h-2V5zM1 1v2h2V1H1zm2 3H1v2h2V4zM1 8v2h2V8H1zm2 3H1v2h2v-2z"/></svg>
                  Videos
              </a>
           </li>
           <li class="nav-item">
              <a class="nav-link" href="/profile">
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 16 16" style="margin-right:4px"><path d="M8 8a3 3 0 1 0 0-6 3 3 0 0 0 0 6zm2-3a2 2 0 1 1-4 0 2 2 0 0 1 4 0zm4 8c0 1-1 1-1 1H3s-1 0-1-1 1-4 6-4 6 3 6 4zm-1-.004c-.001-.246-.154-.986-.832-1.664C11.516 10.68 10.289 10 8 10c-2.29 0-3.516.68-4.168 1.332-.678.678-.83 1.418-.832 1.664h10z"/></svg>
                  Profile
              </a>
           </li>
           <li class="nav-item">
              <a class="nav-link" href="/logout">
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 16 16" style="margin-right:4px"><path fill-rule="evenodd" d="M10 12.5a.5.5 0 0 1-.5.5h-8a.5.5 0 0 1-.5-.5v-9a.5.5 0 0 1 .5-.5h8a.5.5 0 0 1 .5.5v2a.5.5 0 0 0 1 0v-2A1.5 1.5 0 0 0 9.5 2h-8A1.5 1.5 0 0 0 0 3.5v9A1.5 1.5 0 0 0 1.5 14h8a1.5 1.5 0 0 0 1.5-1.5v-2a.5.5 0 0 0-1 0v2z"/><path fill-rule="evenodd" d="M15.854 8.354a.5.5 0 0 0 0-.708l-3-3a.5.5 0 0 0-.708.708L14.293 7.5H5.5a.5.5 0 0 0 0 1h8.793l-2.147 2.146a.5.5 0 0 0 .708.708l3-3z"/></svg>
                  Logout
              </a>
           </li>"#
    } else {
        r#"<li class="nav-item">
              <a class="nav-link" href="/login">
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 16 16" style="margin-right:4px"><path fill-rule="evenodd" d="M6 3.5a.5.5 0 0 1 .5-.5h8a.5.5 0 0 1 .5.5v9a.5.5 0 0 1-.5.5h-8a.5.5 0 0 1-.5-.5v-2a.5.5 0 0 0-1 0v2A1.5 1.5 0 0 0 6.5 14h8a1.5 1.5 0 0 0 1.5-1.5v-9A1.5 1.5 0 0 0 14.5 2h-8A1.5 1.5 0 0 0 5 3.5v2a.5.5 0 0 0 1 0v-2z"/><path fill-rule="evenodd" d="M11.854 8.354a.5.5 0 0 0 0-.708l-3-3a.5.5 0 0 0-.708.708L10.293 7.5H1.5a.5.5 0 0 0 0 1h8.793l-2.147 2.146a.5.5 0 0 0 .708.708l3-3z"/></svg>
                  Login
              </a>
           </li>
           <li class="nav-item">
              <a class="nav-link" href="/register">
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 16 16" style="margin-right:4px"><path d="M6 8a3 3 0 1 0 0-6 3 3 0 0 0 0 6zm2-3a2 2 0 1 1-4 0 2 2 0 0 1 4 0zm4 8c0 1-1 1-1 1H1s-1 0-1-1 1-4 6-4 6 3 6 4zm-1-.004c-.001-.246-.154-.986-.832-1.664C9.516 10.68 8.289 10 6 10c-2.29 0-3.516.68-4.168 1.332-.678.678-.83 1.418-.832 1.664h10z"/><path fill-rule="evenodd" d="M13.5 5a.5.5 0 0 1 .5.5V7h1.5a.5.5 0 0 1 0 1H14v1.5a.5.5 0 0 1-1 0V8h-1.5a.5.5 0 0 1 0-1H13V5.5a.5.5 0 0 1 .5-.5z"/></svg>
                  Register
              </a>
           </li>"#
    };

    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title} — CalmControl</title>
    <link
        href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css"
        rel="stylesheet"
        integrity="sha384-T3c6CoIi6uLrA9TneNEoa7RxnatzjcDSCmG1MXxSR1GAsXEV/Dwwykc2MPK8M2HN"
        crossorigin="anonymous">
    {CSS}
</head>
<body>
<nav class="navbar navbar-expand-lg navbar-dark mb-5">
    <div class="container">
        <a class="navbar-brand text-white" href="/">&#127807; CalmControl</a>
        <button class="navbar-toggler" type="button"
                data-bs-toggle="collapse" data-bs-target="#navMain"
                aria-controls="navMain" aria-expanded="false" aria-label="Toggle navigation">
            <span class="navbar-toggler-icon"></span>
        </button>
        <div class="collapse navbar-collapse" id="navMain">
            <ul class="navbar-nav ms-auto gap-1">
                {nav_items}
            </ul>
        </div>
    </div>
</nav>
<main class="container pb-5">
    {content}
</main>
<script
    src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/js/bootstrap.bundle.min.js"
    integrity="sha384-C6RzsynM9kWDrMNeT87bh95OGNyZPhcTNXj1NW7RuBCsyN/o0jlpcV8Qyq46cDfL"
    crossorigin="anonymous"></script>
</body>
</html>"##
    )
}

// ── Alert helper ───────────────────────────────────────────────────────────────

fn error_alert(msg: &str) -> String {
    format!(
        r#"<div class="alert alert-danger d-flex align-items-center gap-2 rounded-3 mb-4" role="alert">
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" fill="currentColor" viewBox="0 0 16 16">
                <path d="M8.982 1.566a1.13 1.13 0 0 0-1.96 0L.165 13.233c-.457.778.091 1.767.98 1.767h13.713c.889 0 1.438-.99.98-1.767L8.982 1.566zM8 5c.535 0 .954.462.9.995l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 5.995A.905.905 0 0 1 8 5zm.002 6a1 1 0 1 1 0 2 1 1 0 0 1 0-2z"/>
            </svg>
            <span>{msg}</span>
        </div>"#
    )
}

// ── Login page ─────────────────────────────────────────────────────────────────

pub fn login_page(error: Option<&str>) -> String {
    let alert = error.map(|e| error_alert(e)).unwrap_or_default();

    let content = format!(
        r#"<div class="row justify-content-center">
    <div class="col-12 col-sm-10 col-md-7 col-lg-5 col-xl-4">

        <div class="text-center mb-4">
            <div style="font-size:3rem;line-height:1">&#127807;</div>
            <h2 class="fw-bold text-calm mt-2 mb-1">Welcome back</h2>
            <p class="text-muted mb-0">Sign in to your CalmControl account</p>
        </div>

        <div class="card p-4 p-md-5">
            {alert}
            <form method="POST" action="/login" novalidate>
                <div class="mb-3">
                    <label class="form-label" for="email">Email address</label>
                    <input
                        type="email" class="form-control" id="email" name="email"
                        placeholder="you@example.com" autocomplete="email" required>
                </div>
                <div class="mb-4">
                    <label class="form-label" for="password">Password</label>
                    <input
                        type="password" class="form-control" id="password" name="password"
                        placeholder="&bull;&bull;&bull;&bull;&bull;&bull;&bull;&bull;"
                        autocomplete="current-password" required>
                </div>
                <button type="submit" class="btn btn-calm w-100 py-2">
                    Sign In
                </button>
            </form>

            <hr class="my-4">
            <p class="text-center text-muted mb-0" style="font-size:.95rem">
                Don&apos;t have an account?
                <a href="/register" class="text-calm fw-semibold">Create one</a>
            </p>
        </div>

    </div>
</div>"#
    );

    base_layout("Login", &content, false)
}

// ── Register page ──────────────────────────────────────────────────────────────

pub fn register_page(error: Option<&str>) -> String {
    let alert = error.map(|e| error_alert(e)).unwrap_or_default();

    let content = format!(
        r#"<div class="row justify-content-center">
    <div class="col-12 col-sm-10 col-md-8 col-lg-6 col-xl-5">

        <div class="text-center mb-4">
            <div style="font-size:3rem;line-height:1">&#127807;</div>
            <h2 class="fw-bold text-calm mt-2 mb-1">Create your account</h2>
            <p class="text-muted mb-0">Start your journey to calm &amp; clarity</p>
        </div>

        <div class="card p-4 p-md-5">
            {alert}
            <form method="POST" action="/register" novalidate>
                <div class="mb-3">
                    <label class="form-label" for="name">Full Name</label>
                    <input
                        type="text" class="form-control" id="name" name="name"
                        placeholder="Your name" autocomplete="name" required>
                </div>
                <div class="mb-3">
                    <label class="form-label" for="email">Email address</label>
                    <input
                        type="email" class="form-control" id="email" name="email"
                        placeholder="you@example.com" autocomplete="email" required>
                </div>
                <div class="mb-3">
                    <label class="form-label" for="password">Password</label>
                    <input
                        type="password" class="form-control" id="password" name="password"
                        placeholder="Minimum 8 characters"
                        autocomplete="new-password" required>
                </div>
                <div class="mb-4">
                    <label class="form-label" for="confirm_password">Confirm Password</label>
                    <input
                        type="password" class="form-control"
                        id="confirm_password" name="confirm_password"
                        placeholder="Repeat your password"
                        autocomplete="new-password" required>
                </div>
                <button type="submit" class="btn btn-calm w-100 py-2">
                    Create Account
                </button>
            </form>

            <hr class="my-4">
            <p class="text-center text-muted mb-0" style="font-size:.95rem">
                Already have an account?
                <a href="/login" class="text-calm fw-semibold">Sign in</a>
            </p>
        </div>

    </div>
</div>"#
    );

    base_layout("Register", &content, false)
}

// ── Dashboard page ─────────────────────────────────────────────────────────────

pub fn dashboard_page(user: &User, stats: &DashboardStats, weekly: &WeeklyMinutes) -> String {
    let first_name = user.name.split_whitespace().next().unwrap_or(&user.name);
    let bars = weekly_bars(weekly);
    let sessions_today = stats.sessions_today;
    let streak = stats.streak;
    let total_minutes = stats.total_minutes;
    let tip = weekly_tip(stats);

    let content = format!(
        r#"<!-- Header row -->
<div class="d-flex flex-wrap align-items-center justify-content-between gap-3 mb-5">
    <div>
        <h1 class="fw-bold text-calm mb-1">Good day, {first_name}! &#128075;</h1>
        <p class="text-muted mb-0">Here&apos;s your CalmControl overview</p>
    </div>
    <a href="/profile" class="btn btn-calm px-4">
        &#128100;&nbsp; My Profile
    </a>
</div>

<!-- Stat cards -->
<div class="row g-4 mb-5">
    <div class="col-12 col-sm-4">
        <div class="stat-card h-100">
            <div class="d-flex justify-content-between align-items-start mb-3">
                <span class="fw-semibold" style="font-size:1.05rem">Sessions Today</span>
                <span style="font-size:1.5rem">&#128197;</span>
            </div>
            <div class="display-6 fw-bold">{sessions_today}</div>
            <small style="opacity:.75">Completed sessions</small>
        </div>
    </div>
    <div class="col-12 col-sm-4">
        <div class="stat-card h-100" style="background:linear-gradient(135deg,#1a6985,#2196a6)">
            <div class="d-flex justify-content-between align-items-start mb-3">
                <span class="fw-semibold" style="font-size:1.05rem">Day Streak</span>
                <span style="font-size:1.5rem">&#128293;</span>
            </div>
            <div class="display-6 fw-bold">{streak}</div>
            <small style="opacity:.75">Consecutive days</small>
        </div>
    </div>
    <div class="col-12 col-sm-4">
        <div class="stat-card h-100" style="background:linear-gradient(135deg,#7b3f8c,#9b59b6)">
            <div class="d-flex justify-content-between align-items-start mb-3">
                <span class="fw-semibold" style="font-size:1.05rem">Mindful Minutes</span>
                <span style="font-size:1.5rem">&#9200;</span>
            </div>
            <div class="display-6 fw-bold">{total_minutes}</div>
            <small style="opacity:.75">Total time practised</small>
        </div>
    </div>
</div>

<!-- Quick-start & chart row -->
<div class="row g-4">
    <div class="col-12 col-md-6">
        <div class="card p-4 h-100">
            <h5 class="fw-bold text-calm mb-3">&#9889;&nbsp; Quick Start</h5>
            <div class="d-grid gap-2">
                <a href="/breathe" class="btn btn-outline-success rounded-3 text-start py-3 px-4" style="border-color:#cde8d6">
                    &#128168;&nbsp; 5-min Breathing Exercise
                </a>
                <a href="/meditate" class="btn btn-outline-success rounded-3 text-start py-3 px-4" style="border-color:#cde8d6">
                    &#127774;&nbsp; 10-min Guided Meditation
                </a>
                <a href="/journal" class="btn btn-outline-success rounded-3 text-start py-3 px-4" style="border-color:#cde8d6">
                    &#128221;&nbsp; Daily Mood Journal
                </a>
            </div>
        </div>
    </div>

    <div class="col-12 col-md-6">
        <div class="card p-4 h-100">
            <h5 class="fw-bold text-calm mb-3">&#128202;&nbsp; Weekly Activity</h5>
            <div class="bar-wrap">
                {bars}
            </div>
            <div class="d-flex justify-content-between mt-2">
                <small class="text-muted">Mon</small>
                <small class="text-muted">Tue</small>
                <small class="text-muted">Wed</small>
                <small class="text-muted">Thu</small>
                <small class="text-muted">Fri</small>
                <small class="text-muted">Sat</small>
                <small class="text-muted">Sun</small>
            </div>
            <p class="text-muted mt-3 mb-0" style="font-size:.85rem">
                {tip}
            </p>
        </div>
    </div>
</div>"#
    );

    base_layout("Dashboard", &content, true)
}

/// Renders seven bar chart columns driven by real weekly data.
fn weekly_bars(minutes: &WeeklyMinutes) -> String {
    let max = *minutes.iter().max().unwrap_or(&0);
    minutes
        .iter()
        .map(|&m| {
            let h = if max == 0 {
                5u64
            } else {
                ((m as f64 / max as f64) * 90.0 + 10.0) as u64
            };
            let label = if m == 0 {
                "No activity".to_string()
            } else {
                format!("{m} min")
            };
            let active = if m > 0 { " active-bar" } else { "" };
            format!(r#"<div class="bar{active}" style="height:{h}%" title="{label}"></div>"#)
        })
        .collect::<Vec<_>>()
        .join("\n                ")
}

fn weekly_tip(stats: &DashboardStats) -> &'static str {
    if stats.sessions_today == 0 {
        "&#128161; Complete your first session today to start tracking progress!"
    } else if stats.streak >= 7 {
        "&#127942; Amazing! You're on a 7+ day streak. Keep it up!"
    } else if stats.streak >= 3 {
        "&#128293; You're on a roll! Keep the streak going."
    } else {
        "&#10024; Great work today! Every session counts."
    }
}

// ── Breathing page ─────────────────────────────────────────────────────────────

pub fn breathe_page() -> String {
    let content = r#"<div class="row justify-content-center">
    <div class="col-12 col-md-7 col-lg-5 text-center">

        <h2 class="fw-bold text-calm mb-1">&#128168;&nbsp; Breathing Exercise</h2>
        <p class="text-muted mb-0">Box breathing &mdash; 4 seconds each phase &bull; 5 cycles</p>

        <!-- Breathing circle -->
        <div id="breath-circle" class="breath-circle">&#127807;</div>

        <!-- Phase label + countdown -->
        <div id="phase-text" class="fw-bold fs-4 mb-1" style="color:var(--calm-mid)">Get Ready&hellip;</div>
        <div id="countdown" class="display-6 fw-bold text-calm mb-3">&nbsp;</div>

        <!-- Progress -->
        <div class="d-flex justify-content-between mb-1">
            <small class="text-muted">Cycle <span id="cycle-count">0</span> of 5</small>
            <small class="text-muted" id="progress-label">0%</small>
        </div>
        <div class="calm-progress mb-4">
            <div id="progress-bar" class="calm-progress-bar" style="width:0%"></div>
        </div>

        <!-- Complete banner (hidden until done) -->
        <div id="session-done" class="session-done-banner">
            <div style="font-size:2.5rem">&#127881;</div>
            <h5 class="fw-bold text-calm mt-2 mb-1">Session Complete!</h5>
            <p class="text-muted mb-3">You completed 5 rounds of box breathing. Well done!</p>
            <form method="POST" action="/breathe/complete">
                <button type="submit" class="btn btn-calm px-5 py-2">
                    &#10003;&nbsp; Save &amp; Return to Dashboard
                </button>
            </form>
        </div>

        <!-- Back link -->
        <div class="mt-4">
            <a href="/dashboard" class="text-muted" style="font-size:.9rem">&#8592; Back to Dashboard</a>
        </div>

    </div>
</div>

<script>
(function () {
    const PHASES = [
        { name: "Inhale",  seconds: 4, expand: true  },
        { name: "Hold",    seconds: 4, expand: true  },
        { name: "Exhale",  seconds: 4, expand: false },
        { name: "Hold",    seconds: 4, expand: false },
    ];
    const TOTAL_CYCLES = 5;
    const TOTAL_STEPS  = TOTAL_CYCLES * PHASES.length; // 20

    let cycle    = 0;
    let phaseIdx = 0;
    let tick     = PHASES[0].seconds; // seconds remaining in current phase

    const circle      = document.getElementById('breath-circle');
    const phaseText   = document.getElementById('phase-text');
    const countdownEl = document.getElementById('countdown');
    const cycleEl     = document.getElementById('cycle-count');
    const progressBar = document.getElementById('progress-bar');
    const progressLbl = document.getElementById('progress-label');
    const doneDiv     = document.getElementById('session-done');

    function applyCircle(expand) {
        if (expand) {
            circle.classList.add('expand');
        } else {
            circle.classList.remove('expand');
        }
    }

    function render() {
        const phase = PHASES[phaseIdx];
        phaseText.textContent = phase.name;
        countdownEl.textContent = tick;
        cycleEl.textContent = cycle + 1;
        applyCircle(phase.expand);

        const completedSteps = cycle * PHASES.length + phaseIdx;
        const pct = Math.round(completedSteps / TOTAL_STEPS * 100);
        progressBar.style.width = pct + '%';
        progressLbl.textContent = pct + '%';
    }

    // Small delay so first CSS transition fires
    setTimeout(() => {
        render();

        const timer = setInterval(() => {
            tick--;
            if (tick <= 0) {
                phaseIdx++;
                if (phaseIdx >= PHASES.length) {
                    phaseIdx = 0;
                    cycle++;
                }
                if (cycle >= TOTAL_CYCLES) {
                    clearInterval(timer);
                    progressBar.style.width = '100%';
                    progressLbl.textContent = '100%';
                    phaseText.textContent = 'Well done!';
                    countdownEl.textContent = '';
                    doneDiv.style.display = 'block';
                    return;
                }
                tick = PHASES[phaseIdx].seconds;
            }
            render();
        }, 1000);
    }, 300);
}());
</script>"#;

    base_layout("Breathing Exercise", content, true)
}

// ── Meditation page ─────────────────────────────────────────────────────────────

pub fn meditate_page() -> String {
    let content = r#"<div class="row justify-content-center">
    <div class="col-12 col-md-7 col-lg-5 text-center">

        <h2 class="fw-bold mb-1" style="color:#512da8">&#127774;&nbsp; Guided Meditation</h2>
        <p class="text-muted mb-0">Find a comfortable position, close your eyes, and breathe naturally</p>

        <!-- Meditation orb -->
        <div class="meditate-orb">&#129445;</div>

        <!-- Timer display -->
        <div id="timer-display" class="display-4 fw-bold mb-1" style="color:#512da8; font-variant-numeric: tabular-nums;">10:00</div>
        <p id="timer-label" class="text-muted mb-3">remaining</p>

        <!-- Progress -->
        <div class="calm-progress mb-2" style="background:#ede7f6;">
            <div id="progress-bar" class="calm-progress-bar" style="width:0%; background:linear-gradient(90deg,#7b3f8c,#b39ddb);"></div>
        </div>

        <!-- Cycling affirmation -->
        <p id="affirmation" class="fst-italic text-muted mb-4" style="font-size:.95rem; min-height:1.5em;">&ldquo;You are present. You are calm.&rdquo;</p>

        <!-- Complete banner -->
        <div id="session-done" class="session-done-banner" style="border-color:#b39ddb; background:linear-gradient(135deg,#ede7f6,#d1c4e9);">
            <div style="font-size:2.5rem">&#129309;</div>
            <h5 class="fw-bold mt-2 mb-1" style="color:#512da8">Meditation Complete!</h5>
            <p class="text-muted mb-3">10 mindful minutes. Your mind thanks you.</p>
            <form method="POST" action="/meditate/complete">
                <button type="submit" class="btn px-5 py-2"
                        style="background:linear-gradient(135deg,#7b3f8c,#9b59b6);color:#fff;border:none;border-radius:10px;font-weight:500;">
                    &#10003;&nbsp; Save &amp; Return to Dashboard
                </button>
            </form>
        </div>

        <!-- Skip / complete early -->
        <form method="POST" action="/meditate/complete" class="mt-3" id="skip-form" style="display:none;">
            <button type="submit" class="btn btn-sm"
                    style="border:1.5px solid #9b59b6;color:#9b59b6;border-radius:8px;font-size:.85rem;">
                &#9654;&nbsp; Mark Complete Early
            </button>
        </form>

        <div class="mt-3">
            <a href="/dashboard" class="text-muted" style="font-size:.9rem">&#8592; Back to Dashboard</a>
        </div>

    </div>
</div>

<script>
(function () {
    const TOTAL = 600; // 10 minutes in seconds
    let remaining = TOTAL;
    let started = false;

    const affirmations = [
        "\u201cYou are present. You are calm.\u201d",
        "\u201cLet thoughts pass like clouds.\u201d",
        "\u201cBreathe in peace, breathe out tension.\u201d",
        "\u201cThis moment is enough.\u201d",
        "\u201cYou are safe. You are still.\u201d",
        "\u201cEvery breath brings clarity.\u201d",
        "\u201cYou deserve this time for yourself.\u201d",
    ];
    let affirmIdx = 0;

    const timerEl   = document.getElementById('timer-display');
    const labelEl   = document.getElementById('timer-label');
    const bar       = document.getElementById('progress-bar');
    const doneDiv   = document.getElementById('session-done');
    const skipForm  = document.getElementById('skip-form');
    const affirmEl  = document.getElementById('affirmation');

    function fmt(s) {
        const m = Math.floor(s / 60);
        const sec = s % 60;
        return m + ':' + String(sec).padStart(2, '0');
    }

    // Show skip button after 30 seconds
    setTimeout(() => { skipForm.style.display = 'block'; }, 30000);

    // Rotate affirmations every 30s
    setInterval(() => {
        affirmIdx = (affirmIdx + 1) % affirmations.length;
        affirmEl.style.opacity = '0';
        setTimeout(() => {
            affirmEl.textContent = affirmations[affirmIdx];
            affirmEl.style.opacity = '1';
        }, 500);
    }, 30000);
    affirmEl.style.transition = 'opacity .5s';

    const timer = setInterval(() => {
        remaining--;
        timerEl.textContent = fmt(remaining);
        bar.style.width = ((TOTAL - remaining) / TOTAL * 100) + '%';

        if (remaining <= 0) {
            clearInterval(timer);
            timerEl.textContent = '0:00';
            labelEl.textContent = 'complete';
            skipForm.style.display = 'none';
            doneDiv.style.display = 'block';
        }
    }, 1000);
}());
</script>"#;

    base_layout("Guided Meditation", content, true)
}

// ── Journal page ────────────────────────────────────────────────────────────────

pub fn journal_page(error: Option<&str>) -> String {
    let alert = error.map(|e| error_alert(e)).unwrap_or_default();

    let content = format!(
        r#"{alert}

<div class="row justify-content-center">
    <div class="col-12 col-md-8 col-lg-6">

        <h2 class="fw-bold text-calm mb-1">&#128221;&nbsp; Daily Mood Journal</h2>
        <p class="text-muted mb-4">Take a moment to check in with yourself</p>

        <form method="POST" action="/journal">

            <!-- Mood picker -->
            <div class="card p-4 mb-4">
                <h5 class="fw-bold text-calm mb-3">How are you feeling right now?</h5>
                <div class="d-flex justify-content-around">
                    <div class="mood-option">
                        <input type="radio" name="mood" id="m1" value="1" required>
                        <label for="m1" title="Struggling">&#128542;</label>
                    </div>
                    <div class="mood-option">
                        <input type="radio" name="mood" id="m2" value="2">
                        <label for="m2" title="Not great">&#128533;</label>
                    </div>
                    <div class="mood-option">
                        <input type="radio" name="mood" id="m3" value="3">
                        <label for="m3" title="Okay">&#128528;</label>
                    </div>
                    <div class="mood-option">
                        <input type="radio" name="mood" id="m4" value="4">
                        <label for="m4" title="Good">&#128512;</label>
                    </div>
                    <div class="mood-option">
                        <input type="radio" name="mood" id="m5" value="5">
                        <label for="m5" title="Great">&#128513;</label>
                    </div>
                </div>
                <p id="mood-label" class="text-muted text-center mt-3 mb-0" style="font-size:.9rem; min-height:1.2em;"></p>
            </div>

            <!-- Notes -->
            <div class="card p-4 mb-4">
                <h5 class="fw-bold text-calm mb-3">Any thoughts to capture? <span class="text-muted fw-normal" style="font-size:.85rem">(optional)</span></h5>
                <textarea
                    name="note"
                    class="form-control"
                    rows="5"
                    placeholder="What's on your mind today? What are you grateful for? What felt hard?"
                    style="resize:vertical;"
                ></textarea>
            </div>

            <div class="d-grid">
                <button type="submit" class="btn btn-calm py-3 fs-5">
                    &#128221;&nbsp; Save Journal Entry
                </button>
            </div>

        </form>

        <div class="text-center mt-4">
            <a href="/dashboard" class="text-muted" style="font-size:.9rem">&#8592; Back to Dashboard</a>
        </div>

    </div>
</div>

<script>
const labels = ['', 'Struggling \u2014 it\'s okay, you showed up', 'Not great \u2014 acknowledging it is the first step', 'Okay \u2014 steady and present', 'Good \u2014 keep that energy', 'Great \u2014 wonderful!'];
document.querySelectorAll('input[name="mood"]').forEach(radio => {{
    radio.addEventListener('change', () => {{
        document.getElementById('mood-label').textContent = labels[radio.value] || '';
    }});
}});
</script>"#
    );

    base_layout("Journal", &content, true)
}

// ── Profile page ───────────────────────────────────────────────────────────────

pub fn profile_page(user: &User) -> String {
    let initials: String = user
        .name
        .split_whitespace()
        .filter_map(|w| w.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase();

    let name = &user.name;
    let email = &user.email;
    let id = &user.id;

    let content = format!(
        r#"<div class="row justify-content-center">
    <div class="col-12 col-md-8 col-lg-6">

        <h2 class="fw-bold text-calm mb-4">&#128100;&nbsp; My Profile</h2>

        <!-- Avatar + name card -->
        <div class="card p-4 mb-4">
            <div class="d-flex align-items-center gap-4 mb-4">
                <div class="profile-avatar">{initials}</div>
                <div>
                    <h4 class="fw-bold mb-1">{name}</h4>
                    <span class="badge"
                          style="background:#d8f3dc;color:#1b4332;border:1px solid #b7e4c7;
                                 border-radius:20px;padding:4px 14px;font-size:.85rem;font-weight:500">
                        &#10003;&nbsp;Active Member
                    </span>
                </div>
            </div>

            <!-- Info rows -->
            <div class="d-flex flex-column gap-3">
                <div class="info-row">
                    <div class="text-muted mb-1" style="font-size:.82rem">&#128100;&nbsp; FULL NAME</div>
                    <div class="fw-semibold">{name}</div>
                </div>
                <div class="info-row">
                    <div class="text-muted mb-1" style="font-size:.82rem">&#9993;&nbsp; EMAIL ADDRESS</div>
                    <div class="fw-semibold">{email}</div>
                </div>
                <div class="info-row">
                    <div class="text-muted mb-1" style="font-size:.82rem">&#128273;&nbsp; USER ID</div>
                    <div class="fw-semibold text-muted" style="font-size:.88rem;word-break:break-all">{id}</div>
                </div>
            </div>
        </div>

        <!-- Action buttons -->
        <div class="d-flex gap-3">
            <a href="/dashboard" class="btn btn-calm flex-fill py-2">
                &#128202;&nbsp; Dashboard
            </a>
            <a href="/logout"
               class="btn flex-fill py-2"
               style="border:1.5px solid #dc3545;color:#dc3545;border-radius:10px;font-weight:500;transition:all .2s"
               onmouseover="this.style.background='#dc3545';this.style.color='#fff'"
               onmouseout="this.style.background='';this.style.color='#dc3545'">
                &#128682;&nbsp; Logout
            </a>
        </div>

    </div>
</div>"#
    );

    base_layout("Profile", &content, true)
}

// ── 404 Not Found page ─────────────────────────────────────────────────────────

pub fn not_found_page() -> String {
    let content = r#"<div class="row justify-content-center text-center">
    <div class="col-12 col-md-6">
        <div style="font-size:6rem;line-height:1">&#127807;</div>
        <h1 class="fw-bold text-calm mt-3 mb-2" style="font-size:5rem">404</h1>
        <h2 class="fw-semibold mb-3">Page Not Found</h2>
        <p class="text-muted mb-5">
            Looks like this path doesn't exist yet.<br>
            Take a breath and head back to somewhere familiar.
        </p>
        <div class="d-flex justify-content-center gap-3">
            <a href="/dashboard" class="btn btn-calm px-4 py-2">
                &#128202;&nbsp; Dashboard
            </a>
            <a href="/login" class="btn py-2 px-4"
               style="border:1.5px solid #2d6a4f;color:#2d6a4f;border-radius:10px;font-weight:500">
                &#128274;&nbsp; Login
            </a>
        </div>
    </div>
</div>"#;

    base_layout("404 Not Found", content, false)
}

// ── Videos browse page ─────────────────────────────────────────────────────────

pub fn videos_page(videos: &[VideoWithUploader]) -> String {
    let video_cards = if videos.is_empty() {
        r#"<div class="col-12 text-center py-5">
    <div style="font-size:4rem">🎬</div>
    <h4 class="fw-bold text-calm mt-3 mb-2">No videos yet</h4>
    <p class="text-muted mb-4">Be the first to share a health video with the community.</p>
    <a href="/videos/new" class="btn btn-calm px-4">&#43; Add First Video</a>
</div>"#
            .to_string()
    } else {
        videos.iter().map(|v| {
            let cat_slug = &v.category;
            let cat_label = category_label(cat_slug);
            let thumb = if v.thumbnail_url.is_empty() {
                let icon = match cat_slug.as_str() {
                    "breathing"     => "🌬️",
                    "meditation"    => "🧘",
                    "nutrition"     => "🥗",
                    "exercise"      => "💪",
                    "mental-health" => "🧠",
                    _               => "🌿",
                };
                format!(r#"<div class="video-thumb">{icon}</div>"#)
            } else {
                format!(r#"<div class="video-thumb"><img src="{}" alt="thumbnail" loading="lazy"></div>"#, v.thumbnail_url)
            };
            let desc = if v.description.is_empty() {
                String::new()
            } else {
                let short: String = v.description.chars().take(90).collect();
                let ellipsis = if v.description.len() > 90 { "…" } else { "" };
                format!(r#"<p class="text-muted mb-3" style="font-size:.88rem;line-height:1.5">{short}{ellipsis}</p>"#)
            };
            let id = &v.id;
            let title = &v.title;
            let uploader = &v.uploader_name;
            let date = &v.created_at[..10];
            format!(r#"<div class="col-12 col-sm-6 col-lg-4 video-item" data-category="{cat_slug}">
    <div class="video-card">
        <a href="/videos/{id}" style="text-decoration:none;color:inherit;">
            {thumb}
            <div class="p-3">
                <span class="category-badge cat-{cat_slug} mb-2">{cat_label}</span>
                <h6 class="fw-bold mt-2 mb-1" style="line-height:1.35">{title}</h6>
                {desc}
                <div class="d-flex justify-content-between align-items-center mt-auto">
                    <small class="text-muted">&#128100; {uploader}</small>
                    <small class="text-muted">{date}</small>
                </div>
            </div>
        </a>
    </div>
</div>"#)
        }).collect::<Vec<_>>().join("\n")
    };

    let category_pills = {
        let mut pills = String::from(
            r#"<button class="filter-pill active me-2 mb-2" onclick="filterVideos('all', this)">All</button>"#,
        );
        for (slug, label) in CATEGORIES {
            pills.push_str(&format!(
                r#"<button class="filter-pill me-2 mb-2" onclick="filterVideos('{slug}', this)">{label}</button>"#
            ));
        }
        pills
    };

    let content = format!(
        r#"<!-- Header -->
<div class="d-flex flex-wrap align-items-center justify-content-between gap-3 mb-4">
    <div>
        <h2 class="fw-bold text-calm mb-1">&#127909;&nbsp; Health Videos</h2>
        <p class="text-muted mb-0">Curated wellness content from our community</p>
    </div>
    <a href="/videos/new" class="btn btn-calm px-4">&#43;&nbsp; Add Video</a>
</div>

<!-- Category filter -->
<div class="mb-4">
    {category_pills}
</div>

<!-- Grid -->
<div class="row g-4" id="video-grid">
    {video_cards}
</div>

<script>
function filterVideos(cat, btn) {{
    document.querySelectorAll('.filter-pill').forEach(p => p.classList.remove('active'));
    btn.classList.add('active');
    document.querySelectorAll('.video-item').forEach(el => {{
        el.style.display = (cat === 'all' || el.dataset.category === cat) ? '' : 'none';
    }});
}}
</script>"#
    );

    base_layout("Videos", &content, true)
}

// ── New video form ─────────────────────────────────────────────────────────────

pub fn new_video_page(error: Option<&str>) -> String {
    let alert = error.map(|e| error_alert(e)).unwrap_or_default();

    let category_options = CATEGORIES
        .iter()
        .map(|(slug, label)| format!(r#"<option value="{slug}">{label}</option>"#))
        .collect::<Vec<_>>()
        .join("\n");

    let content = format!(
        r#"{alert}

<div class="row justify-content-center">
    <div class="col-12 col-md-8 col-lg-7">

        <h2 class="fw-bold text-calm mb-1">&#127909;&nbsp; Add a Health Video</h2>
        <p class="text-muted mb-4">Share a wellness video with the community. Paste a direct video URL&nbsp;(.mp4, .webm, etc.).</p>

        <form method="POST" action="/videos">

            <div class="card p-4 mb-4">
                <!-- Title -->
                <div class="mb-3">
                    <label class="form-label" for="title">Title <span class="text-danger">*</span></label>
                    <input type="text" id="title" name="title" class="form-control"
                           placeholder="e.g. 5-Minute Morning Stretch" required maxlength="120">
                </div>

                <!-- Category -->
                <div class="mb-3">
                    <label class="form-label" for="category">Category <span class="text-danger">*</span></label>
                    <select id="category" name="category" class="form-control" required>
                        <option value="" disabled selected>Select a category&hellip;</option>
                        {category_options}
                    </select>
                </div>

                <!-- Video URL -->
                <div class="mb-3">
                    <label class="form-label" for="video_url">Video URL <span class="text-danger">*</span></label>
                    <input type="url" id="video_url" name="video_url" class="form-control"
                           placeholder="https://example.com/video.mp4" required>
                    <div class="form-text">Paste a direct link to an .mp4 or .webm file.</div>
                </div>

                <!-- Thumbnail URL -->
                <div class="mb-3">
                    <label class="form-label" for="thumbnail_url">Thumbnail URL <span class="text-muted fw-normal">(optional)</span></label>
                    <input type="url" id="thumbnail_url" name="thumbnail_url" class="form-control"
                           placeholder="https://example.com/thumb.jpg">
                    <div class="form-text">A preview image for the video card. Leave blank for a default icon.</div>
                </div>

                <!-- Description -->
                <div class="mb-0">
                    <label class="form-label" for="description">Description <span class="text-muted fw-normal">(optional)</span></label>
                    <textarea id="description" name="description" class="form-control" rows="4"
                              placeholder="What does this video cover? Who is it for?" style="resize:vertical;"></textarea>
                </div>
            </div>

            <div class="d-flex gap-3">
                <button type="submit" class="btn btn-calm flex-fill py-3">
                    &#127909;&nbsp; Add Video
                </button>
                <a href="/videos" class="btn flex-fill py-3"
                   style="border:1.5px solid #2d6a4f;color:#2d6a4f;border-radius:10px;font-weight:500;">
                    Cancel
                </a>
            </div>

        </form>

    </div>
</div>"#
    );

    base_layout("Add Video", &content, true)
}

// ── Video player page ──────────────────────────────────────────────────────────

pub fn video_player_page(video: &VideoWithUploader) -> String {
    let cat_slug = &video.category;
    let cat_label = category_label(cat_slug);
    let title = &video.title;
    let description = &video.description;
    let uploader = &video.uploader_name;
    let date = &video.created_at[..10];
    let video_url = &video.video_url;

    let desc_block = if description.is_empty() {
        String::new()
    } else {
        format!(
            r#"<div class="card p-4 mb-4">
    <h5 class="fw-bold text-calm mb-2">About this video</h5>
    <p class="mb-0" style="line-height:1.7">{description}</p>
</div>"#
        )
    };

    let content = format!(
        r#"<div class="row justify-content-center">
    <div class="col-12 col-lg-9">

        <!-- Player -->
        <div class="video-player-wrap mb-4">
            <video controls autoplay preload="metadata">
                <source src="{video_url}">
                Your browser does not support the HTML5 video tag.
            </video>
        </div>

        <!-- Meta -->
        <div class="d-flex flex-wrap align-items-start justify-content-between gap-2 mb-3">
            <div>
                <span class="category-badge cat-{cat_slug} mb-2">{cat_label}</span>
                <h3 class="fw-bold mt-2 mb-1">{title}</h3>
                <small class="text-muted">&#128100; {uploader} &nbsp;&bull;&nbsp; {date}</small>
            </div>
            <a href="/videos" class="btn py-2 px-4"
               style="border:1.5px solid var(--calm-mid);color:var(--calm-mid);border-radius:10px;font-weight:500;white-space:nowrap;">
                &#8592;&nbsp; All Videos
            </a>
        </div>

        {desc_block}

    </div>
</div>"#
    );

    base_layout(title, &content, true)
}
