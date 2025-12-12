<?php

namespace App\Http\Controllers;

class OsdrController extends Controller
{
    public function index()
    {
        return view('osdr', [
            'items' => [],
        ]);
    }

    public function dashboard()
    {
        return view('osdr-dashboard', [
            'items' => [],
        ]);
    }
}
