<script>
    import {addDays, isAfter, parse} from 'date-fns'

    export let time;

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
</script>

<span class={activity}></span>


<style>
    span {
        @apply w-3 h-3 rounded-full inline-block mr-3 self-center flex-shrink-0;
    }

    .high {
        @apply bg-blue-600;
    }

    .medium {
        @apply bg-blue-300;
    }

    .low {
        @apply bg-gray-200;
    }

    .none {
        @apply invisible;
    }
</style>
