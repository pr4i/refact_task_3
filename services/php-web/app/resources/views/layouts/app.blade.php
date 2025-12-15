<!doctype html>
<html lang="ru">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>@yield('title', 'Space Dashboard')</title>

  {{-- небольшие ускорители для CDN --}}
  <link rel="preconnect" href="https://cdn.jsdelivr.net" crossorigin>

<link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css" rel="stylesheet">

{{-- Leaflet (лучше jsdelivr вместо unpkg) --}}
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/leaflet@1.9.4/dist/leaflet.css"/>
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@500;700&display=swap" rel="stylesheet">

<style>
  :root{
    --forest:#062016;
    --forest2:#0b2e1d;
    --forest3:#104b2d;
    --white: #ffffff;
    --glass: rgba(8, 40, 25, .62);
    --glass2: rgba(8, 40, 25, .78);
    --stroke: rgba(255,255,255,.14);
  }

  /* фон страницы: темно-зелёный + белый мягкий градиент */
  body{
    background:
      radial-gradient(900px 420px at 18% 0%, rgba(255,255,255,.22), transparent 60%),
      radial-gradient(700px 420px at 85% 10%, rgba(255,255,255,.12), transparent 65%),
      linear-gradient(180deg, var(--forest) 0%, var(--forest2) 55%, var(--forest) 100%);
  }

  /* NAV: белый градиент с зелёным */
  .topbar{
    background: linear-gradient(90deg, rgba(255,255,255,.98), rgba(255,255,255,.82));
    border-bottom: 1px solid rgba(16,75,45,.25);
    box-shadow: 0 8px 24px rgba(0,0,0,.06);
  }

  .brand{
    font-family: "Space Grotesk", system-ui, -apple-system, Segoe UI, Roboto, Arial, sans-serif;
    font-weight: 700;
    letter-spacing: .3px;
    color: #0b2e1d !important;
  }

  .navbar .nav-link{ color: rgba(6,32,22,.8) !important; }
  .navbar .nav-link:hover{ color: rgba(6,32,22,1) !important; }

  .nav-pill{
    border-radius: 999px;
    padding: .45rem .85rem;
  }
  .nav-pill.active{
    background: linear-gradient(135deg, rgba(16,75,45,.14), rgba(16,75,45,.08));
    border: 1px solid rgba(16,75,45,.18);
    font-weight: 700;
  }

  /* основные блоки (карточки) — темно-зелёное стекло + белый текст */
  .card-dark{
    background: var(--glass);
    border: 1px solid var(--stroke);
    color: var(--white);
    box-shadow: 0 10px 26px rgba(0,0,0,.18);
  }
  .card-dark .text-muted{ color: rgba(255,255,255,.72) !important; }
  .card-dark a{ color: rgba(255,255,255,.9); }
  .card-dark a:hover{ color: #fff; }

  /* маленькие “панели” внутри блоков */
  .panel-dark{
    background: rgba(255,255,255,.06);
    border: 1px solid rgba(255,255,255,.12);
    border-radius: .75rem;
    color: #fff;
  }

  /* list-group на темном фоне */
  .list-dark .list-group-item{
    background: transparent;
    color: #fff;
    border-color: rgba(255,255,255,.14);
  }

  /* кнопки (акцент зелёный) */
  .btn-forest{
    background: var(--forest3);
    border-color: var(--forest3);
    color: #fff;
  }
  .btn-forest:hover{
    background: var(--forest2);
    border-color: var(--forest2);
    color: #fff;
  }
  .btn-outline-forest{
    border-color: rgba(255,255,255,.45);
    color: #fff;
  }
  .btn-outline-forest:hover{
    background: rgba(255,255,255,.10);
    border-color: rgba(255,255,255,.65);
    color: #fff;
  }

  /* карта */
  #map{ height:340px; min-height:340px; background:#e9ecef; border-radius:.75rem; overflow:hidden; }
</style>


  @stack('head')
</head>
<body>

<nav class="navbar navbar-expand-lg topbar py-2 mb-3">
  <div class="container">
    <a class="navbar-brand brand" href="{{ route('dashboard') }}">Space Dashboard</a>

    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#topnav">
      <span class="navbar-toggler-icon"></span>
    </button>

    <div class="collapse navbar-collapse" id="topnav">
      <ul class="navbar-nav ms-auto align-items-lg-center gap-1">
        <li class="nav-item">
          <a class="nav-link nav-pill {{ request()->routeIs('dashboard') ? 'active' : '' }}"
             href="{{ route('dashboard') }}">Главная</a>
        </li>
        <li class="nav-item">
          <a class="nav-link nav-pill {{ request()->routeIs('iss.*') ? 'active' : '' }}"
             href="{{ route('iss.index') }}">ISS</a>
        </li>
        <li class="nav-item">
          <a class="nav-link nav-pill {{ request()->routeIs('osdr.*') ? 'active' : '' }}"
             href="{{ route('osdr.index') }}">OSDR</a>
        </li>
        <li class="nav-item">
          <a class="nav-link nav-pill {{ request()->routeIs('telemetry.*') ? 'active' : '' }}"
             href="{{ route('telemetry.index') }}">Telemetry</a>
        </li>

        @if(\Illuminate\Support\Facades\Route::has('cms.page'))
          <li class="nav-item">
            <a class="nav-link nav-pill {{ request()->routeIs('cms.*') ? 'active' : '' }}"
               href="{{ route('cms.page') }}">CMS</a>
          </li>
        @endif
      </ul>
    </div>
  </div>
</nav>

@yield('content')

<script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"></script>

<script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
<script src="https://cdn.jsdelivr.net/npm/leaflet@1.9.4/dist/leaflet.js"></script>

@stack('scripts')
</body>
</html>
