import type { OperationResultStore } from '@urql/svelte';
import gql from 'graphql-tag';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = {
	[_ in K]?: never;
};
export type Incremental<T> =
	| T
	| { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
export type Omit<T, K extends keyof T> = Pick<T, Exclude<keyof T, K>>;
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
	ID: { input: string; output: string };
	String: { input: string; output: string };
	Boolean: { input: boolean; output: boolean };
	Int: { input: number; output: number };
	Float: { input: number; output: number };
};

export type AlbumEntry = {
	__typename?: 'AlbumEntry';
	artist: ArtistAttr;
	attr: EntryAttr;
	name: Scalars['String']['output'];
	playcount: Scalars['String']['output'];
};

export type ArtistAttr = {
	__typename?: 'ArtistAttr';
	text: Scalars['String']['output'];
};

export type ArtistEntry = {
	__typename?: 'ArtistEntry';
	attr: EntryAttr;
	name: Scalars['String']['output'];
	playcount: Scalars['String']['output'];
};

export type EntryAttr = {
	__typename?: 'EntryAttr';
	rank: Scalars['String']['output'];
};

export type HistoryFmQuery = {
	__typename?: 'HistoryFMQuery';
	getWeeklyCharts: WeeklyChartsQuery;
};

export type Query = {
	__typename?: 'Query';
	historyFm: HistoryFmQuery;
};

export type TrackEntry = {
	__typename?: 'TrackEntry';
	artist: ArtistAttr;
	attr: EntryAttr;
	name: Scalars['String']['output'];
	playcount: Scalars['String']['output'];
};

export type WeeklyAlbumChart = {
	__typename?: 'WeeklyAlbumChart';
	album: Array<AlbumEntry>;
	attr: WeeklyChartAttr;
};

export type WeeklyArtistChart = {
	__typename?: 'WeeklyArtistChart';
	artist: Array<ArtistEntry>;
	attr: WeeklyChartAttr;
};

export type WeeklyChartAttr = {
	__typename?: 'WeeklyChartAttr';
	from: Scalars['String']['output'];
	to: Scalars['String']['output'];
};

export type WeeklyChartsQuery = {
	__typename?: 'WeeklyChartsQuery';
	album: Array<WeeklyAlbumChart>;
	artist: Array<WeeklyArtistChart>;
	track: Array<WeeklyTrackChart>;
};

export type WeeklyChartsQueryAlbumArgs = {
	lastfmUsername: Scalars['String']['input'];
};

export type WeeklyChartsQueryArtistArgs = {
	lastfmUsername: Scalars['String']['input'];
};

export type WeeklyChartsQueryTrackArgs = {
	lastfmUsername: Scalars['String']['input'];
};

export type WeeklyTrackChart = {
	__typename?: 'WeeklyTrackChart';
	attr: WeeklyChartAttr;
	track: Array<TrackEntry>;
};

export type ArtistChartQueryVariables = Exact<{
	username: Scalars['String']['input'];
}>;

export type ArtistChartQuery = {
	__typename?: 'Query';
	historyFm: {
		__typename?: 'HistoryFMQuery';
		getWeeklyCharts: {
			__typename?: 'WeeklyChartsQuery';
			artist: Array<{
				__typename?: 'WeeklyArtistChart';
				attr: { __typename?: 'WeeklyChartAttr'; from: string; to: string };
				artist: Array<{
					__typename?: 'ArtistEntry';
					name: string;
					playcount: string;
					attr: { __typename?: 'EntryAttr'; rank: string };
				}>;
			}>;
		};
	};
};

export const ArtistChartDocument = gql`
	query artistChart($username: String!) {
		historyFm {
			getWeeklyCharts {
				artist(lastfmUsername: $username) {
					attr {
						from
						to
					}
					artist {
						name
						playcount
						attr {
							rank
						}
					}
				}
			}
		}
	}
`;
export type ArtistChartQueryStore = OperationResultStore<
	ArtistChartQuery,
	ArtistChartQueryVariables
>;
