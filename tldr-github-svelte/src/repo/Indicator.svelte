<script>
    import {addDays, isAfter, parse} from 'date-fns'

    export let time;

    let oneWeekAgo = addDays(new Date(), -7);
    let twoWeeksAgo = addDays(new Date(), -14);
    let oneMonthAgo = addDays(new Date(), 30);

    let lastUpdated = parse(time, "yyyy-MM-dd'T'HH:mm:ssxxx", new Date());

    let activity ='none';
    if (isAfter(lastUpdated, oneMonthAgo)) {
        activity = 'low';
    }
    if (isAfter(lastUpdated, twoWeeksAgo)) {
        activity = 'medium';
    }
    if (isAfter(lastUpdated, oneWeekAgo)) {
        activity = 'high';
    }

    let clazz = `dot ${activity}`
</script>

<span class={clazz}></span>


<style>
    .dot {
        height: 10px;
        width: 10px;
        border-radius: 50%;
        display: inline-block;
        margin-right: 10px;

        align-self: center;
        flex-shrink: 0;
    }

    .high {
        background-color: #3182ce;
    }

    .medium {
        background-color: #90cdf4
    }

    .low {
        background-color: #cbd5e0
    }

    .none {
        visibility: hidden;
    }
</style>
