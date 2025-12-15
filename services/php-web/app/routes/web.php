<?php

use Illuminate\Support\Facades\Route;
use Illuminate\Http\Request;

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

Route::get('/telemetry', [TelemetryController::class, 'index'])->name('telemetry.index');

Route::get('/jwst', [JwstController::class, 'index'])->name('jwst.index');

Route::get('/cms/{slug?}', [CmsController::class, 'page'])->name('cms.page');

/*
|--------------------------------------------------------------------------
| API proxy (browser -> Laravel -> rust_iss)
|--------------------------------------------------------------------------
*/
Route::prefix('api')->group(function () {

    // ISS
    Route::get('/iss/last', function (RustApiClient $rust) {
        return response()->json($rust->get('/iss/last'));
    });

    Route::get('/iss/trend', function (Request $r, RustApiClient $rust) {
        $limit = (int) $r->query('limit', 20);
        return response()->json($rust->get('/iss/trend?limit=' . $limit));
    });

    // Space summary
    Route::get('/space/summary', function (RustApiClient $rust) {
        return response()->json($rust->get('/space/summary'));
    });

    // OSDR list
    Route::get('/osdr/list', function (Request $r, OsdrService $service) {
        $limit = (int) $r->query('limit', 20);
        $items = $service->getItems($limit);

        return response()->json(['items' => $items]);
    });

    // OSDR sync
    Route::post('/osdr/sync', function (RustApiClient $rust) {
        return response()->json($rust->get('/osdr/sync'));
    });
});
