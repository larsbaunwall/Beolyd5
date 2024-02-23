
/** Convert polar coordinates (r,θ) to cartesian coordinates (x,y)
 * @param cx - The horizontal center of the circle
 * @param cy - The vertical center of the circle
 * @param radius - The radial distance from the center to the arc point
 * @param angleInDegrees - The theta angle
 * @returns A point (x,y) on the circle arc
 */
function polarToCartesian(cx: number, cy: number, radius: number, angleInDegrees: number) {
    const angleInRadians = (angleInDegrees * Math.PI) / 180.0;
    return {
        x: cx + radius * Math.cos(angleInRadians),
        y: cy + radius * Math.sin(angleInRadians),
    };
}

/** Draw an arc from startAngle to endAngle
 * @return SVG path data
 */
export function drawArc(x: number, y: number, radius: number, startAngle: number, endAngle: number): string {
    const start = polarToCartesian(x, y, radius, endAngle);
    const end = polarToCartesian(x, y, radius, startAngle);
    const largeArcFlag = endAngle - startAngle <= 180 ? '0' : '1';
    const d = [
        'M', start.x, start.y,
        'A', radius, radius, 0, largeArcFlag, 0, end.x, end.y
    ].join(' ');
    return d;
}

/** Horizontal center of the circle aligned with the physical location of the BS5 controller relative to the screen */
export const cx: number = 1147;

/** Vertical center of the circle aligned with the physical location of the BS5 controller relative to the screen */
export const cy: number = 387;

/** Translate a value [min;max] to a number 0-100 relative for that range */
export function translateToNormalizedRange(input: number, min: number, max: number): number {
    return translateToRange(input, min, max, 0, 100);
}

/** Translate a value [min;max] to a number in another range [min;max] relative for that range */
function translateToRange(input: number, fromMin: number, fromMax: number, toMin: number, toMax: number): number {
    return ((input - fromMin) * (toMax - toMin) / (fromMax - fromMin)) + toMin;
}

/** Get the (x,y) point on the arc (+padding) at a given angle */
export function getArcPoint(radius: number, radiusPadding: number, angle: number) {
    return polarToCartesian(cx, cy,radius + radiusPadding, angle);
}

export default { describeArc: drawArc, cx, cy, translateToRange, polarToCartesian, getArcPoint }