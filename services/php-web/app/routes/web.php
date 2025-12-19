<?php

use Illuminate\Support\Facades\Route;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Cache;

use App\Http\Controllers\DashboardController;
use App\Http\Controllers\IssController;
use App\Http\Controllers\OsdrController;
use App\Http\Controllers\AstroController;
use App\Http\Controllers\TelemetryController;
use App\Http\Controllers\JwstController;
use App\Http\Controllers\CmsController;
use App\Http\Controllers\LegacyCsvController;

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

Route::get('/telemetry', [TelemetryController::class, 'index'])->name('telemetry.index');

Route::get('/jwst', [JwstController::class, 'index'])->name('jwst.index');

/*
|--------------------------------------------------------------------------
| Legacy CSV (ВАЖНО: должен быть ДО CMS catch-all)
|--------------------------------------------------------------------------
*/
Route::get('/legacy/csv', [LegacyCsvController::class, 'index'])->name('legacy.csv');
Route::get('/legacy/csv/xlsx', [LegacyCsvController::class, 'downloadXlsx'])->name('legacy.csv.xlsx');

/*
|--------------------------------------------------------------------------
| API proxy (browser -> Laravel -> rust_iss)
|--------------------------------------------------------------------------
*/
Route::prefix('api')->group(function () {

    // ISS last (кэш 5 секунд)
    Route::get('/iss/last', function (RustApiClient $rust) {
        $data = Cache::remember('api:iss:last', 5, fn () => $rust->get('/iss/last'));
        return response()->json($data);
    });

    // ISS trend (кэш 5 секунд, ключ зависит от limit)
    Route::get('/iss/trend', function (Request $r, RustApiClient $rust) {
        $limit = max(2, min(200, (int) $r->query('limit', 30)));
        $data = Cache::remember("api:iss:trend:$limit", 5, fn () => $rust->get("/iss/trend?limit=$limit"));
        return response()->json($data);
    });

    // Space summary (кэш 30 секунд)
    Route::get('/space/summary', function (RustApiClient $rust) {
        $data = Cache::remember('api:space:summary', 30, fn () => $rust->get('/space/summary'));
        return response()->json($data);
    });

    // OSDR list (берём из сервиса — он может сам решать откуда)
    Route::get('/osdr/list', function (Request $r, OsdrService $service) {
        $limit = max(1, min(200, (int) $r->query('limit', 20)));
        return response()->json([
            'items' => $service->getItems($limit),
        ]);
    });
});

/*
|--------------------------------------------------------------------------
| CMS (последним, чтобы не перехватывать другие роуты)
|--------------------------------------------------------------------------
*/
Route::get('/cms/{slug?}', [CmsController::class, 'page'])
    ->where('slug', '.*')
    ->name('cms.page');
