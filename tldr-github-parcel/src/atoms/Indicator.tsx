import * as React from "react";
import {addDays, isAfter, parse} from 'date-fns'

type IndicatorProps = {
    time: string,
}
export function Indicator({time}: IndicatorProps): JSX.Element {

    let oneWeekAgo = addDays(new Date(), -7);
    let twoWeeksAgo = addDays(new Date(), -14);
    let oneMonthAgo = addDays(new Date(), 30);

    let lastUpdated = parse(time, "yyyy-MM-dd'T'HH:mm:ssxxx", new Date());

    let activity = 'none';
    if (isAfter(lastUpdated, oneMonthAgo)) {
        activity = 'low';
    }
    if (isAfter(lastUpdated, twoWeeksAgo)) {
        activity = 'medium';
    }
    if (isAfter(lastUpdated, oneWeekAgo)) {
        activity = 'high';
    }

   return <span className={activity}/>
}
