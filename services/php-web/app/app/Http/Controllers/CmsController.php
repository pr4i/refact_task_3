<?php

namespace App\Http\Controllers;

use Illuminate\Support\Facades\DB;

class CmsController extends Controller
{
    public function page(?string $slug = null)
    {
        // /cms -> список
        if (!$slug) {
            $pages = DB::table('cms_pages')
                ->select(['slug', 'title'])
                ->orderBy('title', 'asc')
                ->get();

            return view('cms.index', [
                'pages' => $pages,
            ]);
        }

        // /cms/{slug} -> страница
        $page = DB::table('cms_pages')
            ->where('slug', $slug)
            ->first();

        return view('cms.page', [
            'page' => $page,
        ]);
    }
}
