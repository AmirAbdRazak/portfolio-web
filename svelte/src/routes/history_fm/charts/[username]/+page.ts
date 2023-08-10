import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { getContextClient, queryStore } from '@urql/svelte';
import { ArtistChartDocument, type ArtistChartQuery } from '../../../../generated/graphql';
import { generate } from '@graphql-codegen/cli';

export const load: PageLoad = async ({ params }) => {
	let username = params.username;

	let client = getContextClient();

	let queryData = await client.query(ArtistChartDocument, { username });

	function generateChart() {
		const rawArtistData: ArtistChartQuery = queryData.data;
		const artistData = rawArtistData.historyFm.getWeeklyCharts.artist;

		const dateRanges = artistData.map((artistEntry) => artistEntry.attr.to);

		console.log(dateRanges);
	}

	// const data = {
	// 	labels: labels,
	// 	datasets: [
	// 		{
	// 			label: 'Dataset 1',
	// 			data: Utils.numbers(NUMBER_CFG),
	// 			borderColor: Utils.CHART_COLORS.red,
	// 			backgroundColor: Utils.transparentize(Utils.CHART_COLORS.red, 0.5),
	// 			yAxisID: 'y'
	// 		},
	// 		{
	// 			label: 'Dataset 2',
	// 			data: Utils.numbers(NUMBER_CFG),
	// 			borderColor: Utils.CHART_COLORS.blue,
	// 			backgroundColor: Utils.transparentize(Utils.CHART_COLORS.blue, 0.5),
	// 			yAxisID: 'y1'
	// 		}
	// 	]
	// };

	generateChart();
};
