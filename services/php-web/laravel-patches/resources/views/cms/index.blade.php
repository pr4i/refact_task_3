@extends('layouts.app')

@section('title', 'CMS')

@section('content')
<div class="container py-4">

  <div class="d-flex flex-wrap justify-content-between align-items-center gap-2 mb-3">
    <div>
      <div class="text-uppercase small text-muted">cms</div>
      <h3 class="mb-0">Страницы</h3>
    </div>
    <a class="btn btn-outline-secondary" href="{{ route('dashboard') }}">← Dashboard</a>
  </div>

  <div class="card shadow-sm border-0">
    <div class="card-body">
      @if(($pages ?? collect())->isEmpty())
        <div class="text-muted">Пока нет страниц. Добавь записи в таблицу <code>cms_pages</code>.</div>
      @else
        <div class="list-group list-group-flush">
          @foreach($pages as $p)
            <a class="list-group-item list-group-item-action d-flex justify-content-between align-items-center"
               href="{{ route('cms.page', ['slug' => $p->slug]) }}">
              <div>
                <div class="fw-semibold">{{ $p->title }}</div>
                <div class="small text-muted">/{{ $p->slug }}</div>
              </div>
              <span class="badge rounded-pill text-bg-light">Открыть</span>
            </a>
          @endforeach
        </div>
      @endif
    </div>
  </div>

</div>
@endsection
