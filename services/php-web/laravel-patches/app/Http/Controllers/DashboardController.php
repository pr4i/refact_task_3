<?php

namespace App\Http\Controllers;

use App\Services\DashboardService;

class DashboardController extends Controller
{
    public function index(DashboardService $service)
    {
        $data = $service->getDashboardData();
        return view("dashboard", $data);
    }
}
