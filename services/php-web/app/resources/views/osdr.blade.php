@extends('layouts.app')

@section('title', 'OSDR')

@section('content')
<div class="py-3">

  <div class="p-4 rounded-4 mb-3" style="background: linear-gradient(135deg, #e7f5ff, #eafbea);">
    <div class="d-flex flex-wrap gap-3 align-items-center justify-content-between">
      <div>
        <div class="text-uppercase small text-muted">nasa osdr</div>
        <h3 class="mb-1">Датасеты + фильтры</h3>
        <div class="text-muted">Поиск, сортировка, лимит, просмотр raw JSON</div>
      </div>
      <div class="d-flex gap-2">
        <a class="btn btn-outline-primary" href="{{ route('dashboard') }}">← Главная</a>
        <a class="btn btn-outline-secondary" href="{{ route('osdr.dashboard') }}">Dashboard</a>
      </div>
    </div>
  </div>

  <form method="GET" class="card p-3 shadow-sm border-0 mb-3" id="filterForm">
    <div class="row g-3 align-items-end">

      <div class="col-md-4">
        <label class="form-label">Поиск</label>
        <input type="text"
               class="form-control"
               name="search"
               value="{{ request('search') }}"
               placeholder="dataset_id или title...">
      </div>

      <div class="col-md-3">
        <label class="form-label">Сортировать по</label>
        <select class="form-select" name="sort">
          <option value="inserted_at" @selected(request('sort','inserted_at')==='inserted_at')>inserted_at</option>
          <option value="updated_at"   @selected(request('sort')==='updated_at')>updated_at</option>
          <option value="title"        @selected(request('sort')==='title')>title</option>
        </select>
      </div>

      <div class="col-md-2">
        <label class="form-label">Порядок</label>
        <select class="form-select" name="order">
          <option value="desc" @selected(request('order','desc')==='desc')>По убыванию</option>
          <option value="asc"  @selected(request('order')==='asc')>По возрастанию</option>
        </select>
      </div>

      <div class="col-md-2">
        <label class="form-label">Количество</label>
        <select class="form-select" name="limit">
          @foreach([20,50,100,200] as $l)
            <option value="{{ $l }}" @selected((int)request('limit',20)===$l)>{{ $l }}</option>
          @endforeach
        </select>
      </div>

      <div class="col-md-1 d-grid">
        <button class="btn btn-success">OK</button>
      </div>

      <div class="col-12 d-flex justify-content-end">
        <a class="btn btn-outline-secondary btn-sm" href="{{ route('osdr.index') }}">Сброс</a>
      </div>
    </div>
  </form>

  <div class="card shadow-sm border-0">
    <div class="card-body p-0">
      <div class="table-responsive">
        <table class="table table-hover align-middle mb-0">
          <thead class="table-light">
            <tr>
              <th style="width:80px">#</th>
              <th style="width:140px">dataset_id</th>
              <th style="min-width:320px">title</th>
              <th style="width:120px">REST_URL</th>
              <th style="width:220px">updated_at</th>
              <th style="width:220px">inserted_at</th>
              <th style="width:90px">raw</th>
            </tr>
          </thead>
          <tbody>
          @forelse($items as $row)
            <tr>
              <td>{{ $row['id'] ?? '—' }}</td>
              <td class="fw-semibold">{{ $row['dataset_id'] ?? '—' }}</td>

              <td class="text-truncate" style="max-width:520px" title="{{ $row['title'] ?? '' }}">
                {{ $row['title'] ?? '—' }}
              </td>

              <td>
                @if(!empty($row['rest_url']))
                  <a href="{{ $row['rest_url'] }}" target="_blank" class="btn btn-sm btn-outline-primary">
                    открыть
                  </a>
                @else
                  —
                @endif
              </td>

              <td class="text-muted">{{ $row['updated_at'] ?? '—' }}</td>
              <td class="text-muted">{{ $row['inserted_at'] ?? '—' }}</td>

              <td>
                <button class="btn btn-outline-secondary btn-sm"
                        data-bs-toggle="collapse"
                        data-bs-target="#raw-{{ $row['id'] ?? md5(json_encode($row)) }}">
                  JSON
                </button>
              </td>
            </tr>

            <tr class="collapse bg-light" id="raw-{{ $row['id'] ?? md5(json_encode($row)) }}">
              <td colspan="7">
                <pre class="p-3 mb-0" style="max-height:320px; overflow:auto;">{{ json_encode($row['raw'] ?? [], JSON_PRETTY_PRINT|JSON_UNESCAPED_SLASHES|JSON_UNESCAPED_UNICODE) }}</pre>
              </td>
            </tr>
          @empty
            <tr>
              <td colspan="7" class="text-center text-muted py-4">Нет данных</td>
            </tr>
          @endforelse
          </tbody>
        </table>
      </div>
    </div>
  </div>

  <div class="small text-muted mt-2">
    Источник: <code>/osdr/list</code> (через rust_iss)
  </div>

</div>
@endsection
