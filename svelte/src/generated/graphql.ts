import type { OperationResultStore } from '@urql/svelte';
import gql from 'graphql-tag';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = {
	[K in keyof T]: T[K];
};
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & {
	[SubKey in K]?: Maybe<T[SubKey]>;
};
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & {
	[SubKey in K]: Maybe<T[SubKey]>;
};
export type MakeEmpty<
	T extends { [key: string]: unknown },
	K extends keyof T
> = {
	[_ in K]?: never;
};
export type Incremental<T> =
	| T
	| {
			[P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never;
	  };
export type Omit<T, K extends keyof T> = Pick<T, Exclude<keyof T, K>>;
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
	ID: { input: string; output: string };
	String: { input: string; output: string };
	Boolean: { input: boolean; output: boolean };
	Int: { input: number; output: number };
	Float: { input: number; output: number };
};

export type ChartDataConfig = {
	__typename?: 'ChartDataConfig';
	datasets: Array<DatasetResult>;
	labels: Array<Scalars['Int']['output']>;
};

export type DatasetResult = {
	__typename?: 'DatasetResult';
	chartEntry: Scalars['String']['output'];
	playcount: Array<Scalars['Int']['output']>;
};

export type HistoryFmQuery = {
	__typename?: 'HistoryFMQuery';
	getWeeklyCharts: WeeklyChartsQuery;
};

export type Query = {
	__typename?: 'Query';
	historyFm: HistoryFmQuery;
};

export type WeeklyChartsQuery = {
	__typename?: 'WeeklyChartsQuery';
	chart: ChartDataConfig;
};

export type WeeklyChartsQueryChartArgs = {
	chartType: Scalars['String']['input'];
	lastfmUsername: Scalars['String']['input'];
	limit: Scalars['Int']['input'];
	offset: Scalars['Int']['input'];
};

export type ChartQueryVariables = Exact<{
	username: Scalars['String']['input'];
	chartType: Scalars['String']['input'];
	limit: Scalars['Int']['input'];
	offset: Scalars['Int']['input'];
}>;

export type ChartQuery = {
	__typename?: 'Query';
	historyFm: {
		__typename?: 'HistoryFMQuery';
		getWeeklyCharts: {
			__typename?: 'WeeklyChartsQuery';
			chart: {
				__typename?: 'ChartDataConfig';
				labels: Array<number>;
				datasets: Array<{
					__typename?: 'DatasetResult';
					chartEntry: string;
					playcount: Array<number>;
				}>;
			};
		};
	};
};

export const ChartDocument = gql`
	query chart(
		$username: String!
		$chartType: String!
		$limit: Int!
		$offset: Int!
	) {
		historyFm {
			getWeeklyCharts {
				chart(
					lastfmUsername: $username
					chartType: $chartType
					limit: $limit
					offset: $offset
				) {
					labels
					datasets {
						chartEntry
						playcount
					}
				}
			}
		}
	}
`;
export type ChartQueryStore = OperationResultStore<
	ChartQuery,
	ChartQueryVariables
>;
