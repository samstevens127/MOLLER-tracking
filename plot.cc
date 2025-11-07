#include <iostream>
#include <fstream>
#include <vector>
#include "TVectorD.h"
#include "TFile.h"
#include "TH1D.h"
#include "TF1.h"
#include "TCanvas.h"

std::pair<TF1*, TF1*> plot_tilt_fit(const std::vector<std::vector<double>>& x,
                                    const std::vector<std::vector<double>>& y,
                                    const std::vector<std::vector<double>>& x_err,
                                    const std::vector<std::vector<double>>& y_err)
{
    TGraph *gr_x_vs_y = new TGraph();
    TGraph *gr_y_vs_x = new TGraph();
    int p1 = 0, p2 = 0;

    const size_t nOuter = x.size();
    for (size_t i = 0; i < nOuter; ++i) {
        if (y.size() <= i || x_err.size() <= i || y_err.size() <= i) continue;
        size_t nPoints = x[i].size();
        nPoints = std::min(nPoints, y[i].size());
        nPoints = std::min(nPoints, x_err[i].size());
        nPoints = std::min(nPoints, y_err[i].size());
        for (size_t j = 0; j < nPoints; ++j) {
            gr_x_vs_y->SetPoint(p1++, y[i][j], x_err[i][j]);
            gr_y_vs_x->SetPoint(p2++, x[i][j], y_err[i][j]);
        }
    }

    gStyle->SetOptFit(1);

    TF1 *fit_x_vs_y = nullptr;
    TF1 *fit_y_vs_x = nullptr;

    // Fit x_err as function of y
    if (gr_x_vs_y->GetN() >= 2) {
        // choose fitting range to cover data X range (here X = y)
        double xmin = gr_x_vs_y->GetXaxis()->GetXmin();
        double xmax = gr_x_vs_y->GetXaxis()->GetXmax();
        // If TGraph axis min/max are not set use data min/max:
        double gxmin =  1e300, gxmax = -1e300;
        for (int ip = 0; ip < gr_x_vs_y->GetN(); ++ip) {
            double gx, gy; gr_x_vs_y->GetPoint(ip, gx, gy);
            if (gx < gxmin) gxmin = gx;
            if (gx > gxmax) gxmax = gx;
        }
        if (gxmin <= gxmax) { xmin = gxmin; xmax = gxmax; }

        fit_x_vs_y = new TF1("fit_x_vs_y", "pol1", xmin, xmax);
        gr_x_vs_y->Fit(fit_x_vs_y, "Q"); // quiet fit
        // Draw (optional)
        TCanvas *c1 = new TCanvas("c1_fit_x_vs_y", "x_err vs y", 700, 500);
        gr_x_vs_y->SetMarkerStyle(20);
        gr_x_vs_y->SetMarkerSize(0.9);
        gr_x_vs_y->SetMarkerColor(kBlue);
        gr_x_vs_y->SetTitle("x_{residual} vs y; y (mm); x residual (mm)");
        gr_x_vs_y->Draw("AP");
        fit_x_vs_y->SetLineColor(kRed);
        fit_x_vs_y->Draw("Same");
    } else if (gr_x_vs_y->GetN() == 1) {
        printf("plot_tilt_fit: only 1 point for x_err vs y — cannot fit a line.\n");
    } else {
        printf("plot_tilt_fit: 0 points for x_err vs y — nothing to fit.\n");
    }

    // Fit y_err as function of x
    if (gr_y_vs_x->GetN() >= 2) {
        double xmin = gr_y_vs_x->GetXaxis()->GetXmin();
        double xmax = gr_y_vs_x->GetXaxis()->GetXmax();
        double gxmin =  1e300, gxmax = -1e300;
        for (int ip = 0; ip < gr_y_vs_x->GetN(); ++ip) {
            double gx, gy; gr_y_vs_x->GetPoint(ip, gx, gy);
            if (gx < gxmin) gxmin = gx;
            if (gx > gxmax) gxmax = gx;
        }
        if (gxmin <= gxmax) { xmin = gxmin; xmax = gxmax; }

        fit_y_vs_x = new TF1("fit_y_vs_x", "pol1", xmin, xmax);
        gr_y_vs_x->Fit(fit_y_vs_x, "Q");
        TCanvas *c2 = new TCanvas("c2_fit_y_vs_x", "y_err vs x", 700, 500);
        gr_y_vs_x->SetMarkerStyle(20);
        gr_y_vs_x->SetMarkerSize(0.9);
        gr_y_vs_x->SetMarkerColor(kBlue);
        gr_y_vs_x->SetTitle("y_{residual} vs x; x (mm); y residual (mm)");
        gr_y_vs_x->Draw("AP");
        fit_y_vs_x->SetLineColor(kRed);
        fit_y_vs_x->Draw("Same");
    } else if (gr_y_vs_x->GetN() == 1) {
        printf("plot_tilt_fit: only 1 point for y_err vs x — cannot fit a line.\n");
    } else {
        printf("plot_tilt_fit: 0 points for y_err vs x — nothing to fit.\n");
    }

    return std::make_pair(fit_x_vs_y, fit_y_vs_x);
}

double extract_gauss_mean(const string var_name, vector<double> &v1, const double plt_lower,  const double plt_upper, bool show_plot = true, const uint32_t num_bins = 100)
{
	// Plot distribution of errors.
	string hist_title = "Distribution of errors of " + var_name;
	string hist_name = "error_dist_" + var_name;
	TH1D *error_distribution = new TH1D(hist_name.c_str(), hist_title.c_str(), num_bins, plt_lower,plt_upper);


	// fill distribution
	for (auto &elem: v1){
		error_distribution->Fill(elem);
	}

	gStyle->SetOptFit(1); // show mean on legend

	//fit to gaussian
	TF1 *g1 = new TF1 ("g1","gaus",plt_lower,plt_upper);
	if (show_plot)
		error_distribution->Fit(g1, "rQ");
	else
		error_distribution->Fit(g1, "RQ0");



	auto mean = g1->GetParameter(1);


	if (show_plot){
		TCanvas *c1 = new TCanvas();
		error_distribution->GetXaxis()->SetTitle("residual (mm)");
		error_distribution->GetYaxis()->SetTitle("counts");
		error_distribution->Draw();
	}

	if (!show_plot)
		delete error_distribution;
	return mean;
}

void plot()
{
	std::string filename = "corrected_x_y.txt";
	std::ifstream file(filename);
	double c1,x1,y1,z1,x2,y2,z2,x3,y3,z3;

	std::vector<double> x1_vec, x2_vec, x3_vec,y1_vec, y2_vec, y3_vec,z1_vec, z2_vec, z3_vec;

	while (file >> c1>>x1>>y1>>z1>>x2>>y2>>z2>>x3>>y3>>z3)
	{
		cout << x1 << " " << x2 << " " << x3 << endl;
		x1_vec.push_back(x1);
		x2_vec.push_back(x2);
		x3_vec.push_back(x3);

		y1_vec.push_back(y1);
		y2_vec.push_back(y2);
		y3_vec.push_back(y3);
	}

	vector<vector<double>> x = {x1_vec,x2_vec,x3_vec};
	vector<vector<double>> y = {y1_vec,y2_vec,x3_vec};
	vector<double> z = {z1,z2,z3};
	vector<vector<double>> x_err(3), y_err(3);

	cout << x[0].size() << endl;
	cout << x_err[0].size() << endl;
	extract_gauss_mean("GEM-I",  y[0], -50,50);
	extract_gauss_mean("GEM-II", y[1], -50,50);
	extract_gauss_mean("GEM-III",y[2], -50,50);
}
