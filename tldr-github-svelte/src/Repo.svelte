<script>
  import LastCommit from './LastCommit.svelte';
  import Activity from './Activity.svelte';
  export let repo
</script>


<article class="card horizontal-flex">
  <header class="card-header">
    <div class="card-header-title">
      <p class="grow">{repo.title}</p>
      <i class="icon ion-md-settings" />
    </div>
  </header>
  <div class="card-content grow">
    <div class="content stack">
      {#if repo.lastCommit }
        <LastCommit lastCommit={repo.lastCommit} />
      {:else}
        <p>No commit so far</p>
      {/if}
      {#if repo.activity}
        <Activity activity={repo.activity} />
      {:else}
        <p>No activity so far</p>
      {/if}
    </div>
  </div>
  <footer class="card-footer">
    <p class="is-size-7 card-footer-item">Last update 2min ago</p>
  </footer>
</article>
