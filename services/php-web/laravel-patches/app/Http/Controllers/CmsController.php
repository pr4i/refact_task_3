<?php

namespace App\Http\Controllers;

use Illuminate\Support\Facades\DB;

class CmsController extends Controller
{
    // Роут: /cms/{slug?}
    public function page(?string $slug = null)
    {
        // /cms -> список страниц
        if (!$slug) {
            $pages = DB::table('cms_pages')
                ->select(['slug', 'title'])     // updated_at у тебя НЕТ
                ->orderBy('title', 'asc')
                ->get();

            return view('cms.index', [
                'pages' => $pages,
            ]);
        }

        // /cms/{slug} -> конкретная страница
        $page = DB::table('cms_pages')
            ->where('slug', $slug)
            ->first();

        if (!$page) {
            abort(404);
        }

        return view('cms.page', [
            'page' => $page,
        ]);
    }
}
