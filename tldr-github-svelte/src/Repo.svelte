<script>
  import { fade } from 'svelte/transition';
  import LastCommit from './LastCommit.svelte';
  import Activity from './Activity.svelte';
  import Settings from './settings/Settings.svelte';
  export let repo

  let showSettings = false
</script>

<article transition:fade="{{duration: 500}}" class="card horizontal-flex">
  <header class="card-header">
    <div class="card-header-title">
      <p class="grow">{repo.title}</p>
      <a href="#" on:click|preventDefault={() => showSettings = !showSettings}>
        <i class="icon ion-md-settings" />
      </a>
    </div>
  </header>
  <div class="card-content grow">
    <div class="content stack">
      {#if showSettings }
        <Settings repoId={repo.id} on:repo-deleted/>
      {:else}
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
      {/if}
      </div>
  </div>
  <footer class="card-footer">
    <p class="is-size-7 card-footer-item">Last update 2min ago</p>
  </footer>
</article>
