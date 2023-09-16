import type { ChartDataConfig } from '@/src/generated/graphql';
import type { ChartConfiguration } from 'chart.js';

export type formDataType = {
	chartType: string;
	limit: number;
	offset: number;
	startTimestamp: number;
	endTimestamp: number;
	chartScale: 'linear' | 'logarithmic';
	dateRange: 'Week' | 'Month' | 'Quarter' | 'Year' | 'Custom';
};

const dateRangeMap = {
	Week: 'Weekly',
	Month: 'Monthly',
	Quarter: 'Quarterly',
	Year: 'Annually',
	Custom: 'Custom'
};

function getRandomColor() {
	const red = Math.floor(Math.random() * 128) + 128; // Bias towards higher values (128-255)
	const green = Math.floor(Math.random() * 128) + 128; // Bias towards higher values (128-255)
	const blue = Math.floor(Math.random() * 128) + 128; // Bias towards higher values (128-255)

	const color = `#${((red << 16) | (green << 8) | blue)
		.toString(16)
		.padStart(6, '0')}`;

	return color;
}

export function _getChartConfig(
	chartData: ChartDataConfig,
	formData: formDataType,
	screenSize: number,
	locale: string
): ChartConfiguration {
	return {
		type: 'line',
		data: {
			labels: chartData.labels.map((timestamp) => new Date(timestamp * 1000)),
			datasets: chartData.datasets.map((dataset) => {
				const randColor = getRandomColor();
				return {
					label: dataset.chartEntry,
					data: dataset.playcount,
					borderColor: randColor,
					backgroundColor: randColor,
					pointHoverBackgroundColor: '#1e293b',
					pointHoverBorderColor: randColor,
					pointStyle: 'rectRot',
					pointRadius: 0,
					pointHoverRadius: 6,
					borderWidth: 2,
					tension: 0.15
				};
			})
		},
		options: {
			responsive: true,
			aspectRatio:
				screenSize >= 768
					? 2
					: screenSize >= 640
					? 1.5
					: screenSize >= 420
					? 1
					: screenSize >= 300
					? 0.75
					: 0.5,
			interaction: {
				mode: 'index',
				intersect: false
			},
			scales: {
				x: {
					type: 'timeseries',
					time: {
						displayFormats: {
							quarter: 'MMM YYYY'
						}
					},
					ticks: {
						color: '#ffffff',
						font: {
							family: 'Montserrat'
						},
						callback: (value, index) => {
							if (
								formData.dateRange == 'Year' ||
								formData.dateRange == 'Custom'
							) {
								return index % 12 == 0 || index == 0
									? new Date(value).toLocaleDateString(locale, {
											year: 'numeric',
											month: 'short'
									  })
									: null;
							} else if (formData.dateRange == 'Quarter') {
								return index % 4 == 0 || index == 0
									? new Date(value).toLocaleDateString(locale, {
											year: '2-digit',
											month: 'short'
									  })
									: null;
							} else {
								return new Date(value).toLocaleDateString(locale, {
									year: '2-digit',
									month: 'short',
									day: '2-digit'
								});
							}
						}
					}
				},
				y: {
					type: formData.chartScale,
					min: formData.chartScale == 'linear' ? 0 : 1,
					ticks: {
						maxTicksLimit: 15,
						color: '#ffffff',
						font: {
							family: 'Montserrat'
						}
					}
				}
			},

			plugins: {
				title: {
					display: true,
					text: `${
						formData.dateRange != 'Custom'
							? dateRangeMap[formData.dateRange]
							: new Date(chartData.labels[0] * 1000).toLocaleDateString(
									locale,
									{ year: 'numeric', month: 'short' }
							  ) +
							  ' to ' +
							  new Date(formData.endTimestamp * 1000).toLocaleDateString(
									locale,
									{ year: 'numeric', month: 'short' }
							  )
					} ${formData.chartType} Chart`,
					color: '#ffffff',
					font: {
						family: 'Montserrat'
					}
				},
				tooltip: {
					enabled: true,
					mode: 'index',
					position: 'nearest',
					itemSort: function (a, b) {
						return (b.raw as number) - (a.raw as number);
					},
					titleFont: {
						family: 'Montserrat'
					},
					bodyFont: {
						family: 'Montserrat'
					}
				},
				legend: {
					display: true,
					position: 'bottom',
					labels: {
						font: {
							family: 'Montserrat'
						},
						color: '#ffffff'
					}
				},
				zoom: {
					limits: {
						y: { min: 'original', max: 'original' },
						x: { min: 'original', max: 'original' }
					},
					zoom: {
						wheel: {
							enabled: true,
							modifierKey: 'ctrl'
						},
						pinch: {
							enabled: true
						},
						mode: 'xy'
					},
					pan: {
						enabled: true,
						mode: 'xy'
					}
				}
			}
		}
	};
}
