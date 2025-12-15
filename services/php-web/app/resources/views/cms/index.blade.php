@extends('layouts.app')

@section('title', 'CMS')

@section('content')
<div class="container py-4">
  <h3 class="mb-3">CMS</h3>
  <div class="text-muted mb-3">Страницы из таблицы <code>cms_pages</code></div>

  <div class="card shadow-sm border-0">
    <div class="card-body">
      @if(($pages->count() ?? 0) > 0)
        <div class="list-group">
          @foreach($pages as $p)
            <a class="list-group-item list-group-item-action"
               href="{{ route('cms.page', ['slug' => $p->slug]) }}">
              <div class="fw-semibold">{{ $p->title ?? $p->slug }}</div>
              <div class="small text-muted">/cms/{{ $p->slug }}</div>
            </a>
          @endforeach
        </div>
      @else
        <div class="text-muted">В таблице пока нет страниц.</div>
      @endif
    </div>
  </div>
</div>
@endsection
