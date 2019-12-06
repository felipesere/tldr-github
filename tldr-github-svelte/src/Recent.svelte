<script>
  import Master from './Master.svelte';
  export let master;
</script>

<div>
  <h2 class="recent-activity">Recent activity</h2>
  <Master master={master} />
</div>

<style>
.recent-activity {
  font-size: 1.25rem;
  margin-top: 2rem;
  margin-bottom: 0;
}
</style>
