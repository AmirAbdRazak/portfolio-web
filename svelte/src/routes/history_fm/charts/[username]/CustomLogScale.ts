import {
	Chart,
	Scale,
	Ticks,
	type BubbleDataPoint,
	type ChartTypeRegistry,
	type Point,
	LogarithmicScale
} from 'chart.js';

class LinearScaleBase extends Scale {
	/** @type {number} */
	start!: number;
	/** @type {number} */
	end!: number;
	/** @type {number} */
	_startValue!: number;
	/** @type {number} */
	_endValue!: number;
	_valueRange!: number;
	// @ts-ignore
	parse(raw: any, index: any): number;
	// @ts-ignore
	handleTickRangeOptions(): void;
	// @ts-ignore
	getTickLimit(): number;
	/**
	 * @protected
	 */
	// @ts-ignore
	protected computeTickLimit(): number;
	// @ts-ignore
	getLabelForValue(value: any): string;
}
export default class CustomLogScale extends LogarithmicScale {
	static id = 'customLog';
	static defaults = {
		ticks: {
			callback: Ticks.formatters.logarithmic,
			major: {
				enabled: true
			}
		}
	};
	start: any;
	end: any;
	_startValue: any;
	_valueRange: number;
	_zero: boolean = false;

	constructor(cfg: {
		id: string;
		type: string;
		ctx: any;
		chart: Chart<
			keyof ChartTypeRegistry,
			(number | [number, number] | Point | BubbleDataPoint | null)[],
			unknown
		>;
	}) {
		super(cfg);
		this.start = undefined;
		this.end = undefined;
		this._startValue = undefined;
		this._valueRange = 0;
	}
	parse(raw: any, index: any) {
		const value = LinearScaleBase.prototype.parse.apply(this, [raw, index]);
		if (value === 0) {
			this._zero = true;
			return undefined;
		}
		return isNumberFinite(value) && value > 0 ? value : null;
	}
	determineDataLimits() {
		const { min, max } = this.getMinMax(true);
		// @ts-ignore
		this.min = isNumberFinite(min) ? Math.max(0, min) : null;
		// @ts-ignore
		this.max = isNumberFinite(max) ? Math.max(0, max) : null;
		// @ts-ignore
		if (this.options.beginAtZero) {
			this._zero = true;
		}
		// @ts-ignore
		if (this._zero && this.min !== this._suggestedMin && !isNumberFinite(this._userMin)) {
			this.min =
				min === changeExponent(this.min, 0)
					? changeExponent(this.min, -1)
					: changeExponent(this.min, 0);
		}
		this.handleTickRangeOptions();
	}
	handleTickRangeOptions() {
		const { minDefined, maxDefined } = this.getUserBounds();
		let min = this.min;
		let max = this.max;
		const setMin = (v: number) => (min = minDefined ? min : v);
		const setMax = (v: number) => (max = maxDefined ? max : v);
		if (min === max) {
			if (min <= 0) {
				setMin(1);
				setMax(10);
			} else {
				setMin(changeExponent(min, -1));
				setMax(changeExponent(max, +1));
			}
		}
		if (min <= 0) {
			setMin(changeExponent(max, -1));
		}
		if (max <= 0) {
			setMax(changeExponent(min, +1));
		}
		this.min = min;
		this.max = max;
	}
	buildTicks() {
		const opts = this.options;
		const generationOptions = {
			// @ts-ignore
			min: this._userMin,
			// @ts-ignore
			max: this._userMax
		};
		const ticks = generateTicks(generationOptions, this);
		if (opts.bounds === 'ticks') {
			_setMinAndMaxByKey(ticks, this, 'value');
		}
		if (opts.reverse) {
			ticks.reverse();
			this.start = this.max;
			this.end = this.min;
		} else {
			this.start = this.min;
			this.end = this.max;
		}
		return ticks;
	}
	getLabelForValue(value: any) {
		return value === undefined
			? '0'
			: formatNumber(value, this.chart.options.locale, this.options.ticks.format);
	}
	configure() {
		const start = this.min;
		super.configure();
		this._startValue = log2(start);
		this._valueRange = log2(this.max) - log2(start);
	}
	getPixelForValue(value: number) {
		if (value === undefined || value === 0) {
			value = this.min;
		}
		if (value === null || isNaN(value)) {
			return NaN;
		}
		return this.getPixelForDecimal(
			value === this.min ? 0 : (log2(value) - this._startValue) / this._valueRange
		);
	}
	getValueForPixel(pixel: number) {
		const decimal = this.getDecimalForPixel(pixel);
		return Math.pow(2, this._startValue + decimal * this._valueRange);
	}
}

const isNumberFinite = (value: any) =>
	(typeof value === 'number' || value instanceof Number) && isFinite(+value);
const log2Floor = (v: number) => Math.floor(log2(v));
const changeExponent = (v: number, m: number) => Math.pow(2, log2Floor(v) + m);
const log2 = Math.log2;
function finiteOrDefault(value: number, defaultValue: number) {
	return isNumberFinite(value) ? value : defaultValue;
}

function startExp(min: number, max: number) {
	const range = max - min;
	let rangeExp = log2Floor(range);
	while (steps(min, max, rangeExp) > 10) {
		rangeExp++;
	}
	while (steps(min, max, rangeExp) < 10) {
		rangeExp--;
	}
	return Math.min(rangeExp, log2Floor(min));
}
function steps(min: number, max: number, rangeExp: number) {
	const rangeStep = Math.pow(2, rangeExp);
	const start = Math.floor(min / rangeStep);
	const end = Math.ceil(max / rangeStep);
	return end - start;
}
function isMajor(tickVal: number) {
	const remain = tickVal / Math.pow(2, log2Floor(tickVal));
	return remain === 1;
}

function generateTicks(generationOptions: any, { min, max }: any) {
	min = finiteOrDefault(generationOptions.min, min);
	const ticks = [];
	const minExp = log2Floor(min);
	let exp = startExp(min, max);
	let precision = exp < 0 ? Math.pow(2, Math.abs(exp)) : 1;
	const stepSize = Math.pow(2, exp);
	const base = minExp > exp ? Math.pow(2, minExp) : 0;
	const start = Math.round((min - base) * precision) / precision;
	const offset = Math.floor((min - base) / stepSize / 10) * stepSize * 2;
	let significand = Math.floor((start - offset) / Math.pow(2, exp));
	let value = finiteOrDefault(
		generationOptions.min,
		Math.round((base + offset + significand * Math.pow(2, exp)) * precision) / precision
	);
	while (value < max) {
		ticks.push({
			value,
			major: isMajor(value),
			significand
		});
		if (significand >= 10) {
			significand = significand < 15 ? 15 : 20;
		} else {
			significand++;
		}
		if (significand >= 20) {
			exp++;
			significand = 2;
			precision = exp >= 0 ? 1 : precision;
		}
		value = Math.round((base + offset + significand * Math.pow(2, exp)) * precision) / precision;
	}
	const lastTick = finiteOrDefault(generationOptions.max, value);
	ticks.push({
		value: lastTick,
		major: isMajor(lastTick),
		significand
	});
	return ticks;
}

function _setMinAndMaxByKey(array: any, target: any, property: any) {
	let i, ilen, value;
	for (i = 0, ilen = array.length; i < ilen; i++) {
		value = array[i][property];
		if (!isNaN(value)) {
			target.min = Math.min(target.min, value);
			target.max = Math.max(target.max, value);
		}
	}
}

const intlCache = new Map();
function getNumberFormat(locale: string | undefined, options: any) {
	options = options || {};
	const cacheKey = locale + JSON.stringify(options);
	let formatter = intlCache.get(cacheKey);
	if (!formatter) {
		formatter = new Intl.NumberFormat(locale, options);
		intlCache.set(cacheKey, formatter);
	}
	return formatter;
}
function formatNumber(num: number, locale: string | undefined, options: any) {
	return getNumberFormat(locale, options).format(num);
}
