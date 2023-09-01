import type { ChartConfiguration } from 'chart.js';
import type { PageLoad } from './$types';
import type { ChartDataConfig } from '@/src/generated/graphql';

export type formDataType = {
	chartType: string;
	limit: number;
	offset: number;
	startTimestamp: number;
	endTimestamp: number;
	chartScale: 'linear' | 'logarithmic';
};

export const load: PageLoad = ({ params }) => {
	const username = params.username;
	return { username };
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
					: screenSize >= 300
					? 1
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
						callback: (value, index) =>
							index % 12 == 0 || index == 0
								? new Date(value).toLocaleDateString(locale, {
										year: 'numeric',
										month: 'short'
								  })
								: null
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
					text: `${formData.chartType} All Time Chart`,
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
