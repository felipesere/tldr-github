<script>
  import Recent from './Recent.svelte';
  import PullRequests from './PullRequests.svelte';
  import Issues from './Issues.svelte';
  export let activity;
</script>


<Recent master={activity.master} />
<div class="stack">
  <PullRequests prs={activity.prs} />
  <Issues issues={activity.issues} />
</div>
