<?php

use Illuminate\Support\Facades\Route;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Cache;

use App\Http\Controllers\DashboardController;
use App\Http\Controllers\IssController;
use App\Http\Controllers\OsdrController;
use App\Http\Controllers\AstroController;
use App\Http\Controllers\JwstController;
use App\Http\Controllers\CmsController;
use App\Http\Controllers\TelemetryController;

use App\DataSources\RustApiClient;
use App\Services\OsdrService;

/*
|--------------------------------------------------------------------------
| Pages
|--------------------------------------------------------------------------
*/

Route::get('/', fn () => redirect()->route('dashboard'));

Route::get('/dashboard', [DashboardController::class, 'index'])->name('dashboard');

Route::get('/iss', [IssController::class, 'index'])->name('iss.index');

Route::get('/osdr', [OsdrController::class, 'index'])->name('osdr.index');
Route::get('/osdr/dashboard', [OsdrController::class, 'dashboard'])->name('osdr.dashboard');

Route::get('/astro', [AstroController::class, 'events'])->name('astro.events');

Route::get('/jwst', [JwstController::class, 'index'])->name('jwst.index');

Route::get('/telemetry', [TelemetryController::class, 'index'])->name('telemetry.index');

// CMS: /cms -> index, /cms/{slug} -> page
Route::get('/cms/{slug?}', [CmsController::class, 'page'])->name('cms.page');


/*
|--------------------------------------------------------------------------
| API proxy (browser -> Laravel -> rust_iss)
| Важно: браузер не ходит на http://rust_iss:3000 напрямую.
|--------------------------------------------------------------------------
*/

Route::prefix('api')->group(function () {

    // ISS last (кэш 5 сек)
    Route::get('/iss/last', function (RustApiClient $rust) {
        $data = Cache::remember('iss:last', 5, fn () => $rust->get('/iss/last'));
        return response()->json($data);
    });

    // ISS trend (кэш 10 сек, ключ зависит от limit)
    Route::get('/iss/trend', function (Request $r, RustApiClient $rust) {
        $limit = max(2, min(200, (int) $r->query('limit', 30)));
        $key = "iss:trend:{$limit}";

        $data = Cache::remember($key, 10, fn () => $rust->get('/iss/trend?limit=' . $limit));
        return response()->json($data);
    });

    // Space summary (кэш 60 сек)
    Route::get('/space/summary', function (RustApiClient $rust) {
        $data = Cache::remember('space:summary', 60, fn () => $rust->get('/space/summary'));
        return response()->json($data);
    });

    // OSDR list (кэш 30 сек, зависит от limit)
    Route::get('/osdr/list', function (Request $r, OsdrService $service) {
        $limit = max(1, min(200, (int) $r->query('limit', 20)));
        $key = "osdr:list:{$limit}";

        $items = Cache::remember($key, 30, fn () => $service->getItems($limit));

        return response()->json(['items' => $items]);
    });

    // OSDR sync (без кэша)
    Route::post('/osdr/sync', function (RustApiClient $rust) {
        return response()->json($rust->get('/osdr/sync'));
    });

});
