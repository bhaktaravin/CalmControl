use crate::models::user::User;

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

pub fn dashboard_page(user: &User) -> String {
    let first_name = user.name.split_whitespace().next().unwrap_or(&user.name);
    let bars = weekly_bars();

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
            <div class="display-6 fw-bold">0</div>
            <small style="opacity:.75">Completed sessions</small>
        </div>
    </div>
    <div class="col-12 col-sm-4">
        <div class="stat-card h-100" style="background:linear-gradient(135deg,#1a6985,#2196a6)">
            <div class="d-flex justify-content-between align-items-start mb-3">
                <span class="fw-semibold" style="font-size:1.05rem">Day Streak</span>
                <span style="font-size:1.5rem">&#128293;</span>
            </div>
            <div class="display-6 fw-bold">0</div>
            <small style="opacity:.75">Consecutive days</small>
        </div>
    </div>
    <div class="col-12 col-sm-4">
        <div class="stat-card h-100" style="background:linear-gradient(135deg,#7b3f8c,#9b59b6)">
            <div class="d-flex justify-content-between align-items-start mb-3">
                <span class="fw-semibold" style="font-size:1.05rem">Mindful Minutes</span>
                <span style="font-size:1.5rem">&#9200;</span>
            </div>
            <div class="display-6 fw-bold">0</div>
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
                <button class="btn btn-outline-success rounded-3 text-start py-3 px-4" style="border-color:#cde8d6">
                    &#128168;&nbsp; 5-min Breathing Exercise
                </button>
                <button class="btn btn-outline-success rounded-3 text-start py-3 px-4" style="border-color:#cde8d6">
                    &#127774;&nbsp; 10-min Guided Meditation
                </button>
                <button class="btn btn-outline-success rounded-3 text-start py-3 px-4" style="border-color:#cde8d6">
                    &#128221;&nbsp; Daily Mood Journal
                </button>
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
                &#128161; Complete your first session to start tracking progress!
            </p>
        </div>
    </div>
</div>"#
    );

    base_layout("Dashboard", &content, true)
}

/// Renders seven placeholder bar chart columns.
fn weekly_bars() -> String {
    let heights: [u8; 7] = [20, 45, 60, 30, 75, 50, 10];
    heights
        .iter()
        .map(|h| format!(r#"<div class="bar" style="height:{h}%" title="{h} min"></div>"#))
        .collect::<Vec<_>>()
        .join("\n                ")
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
