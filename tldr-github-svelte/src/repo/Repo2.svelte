<script>
  import { fade } from 'svelte/transition';
  import Github from './Github.svelte';
  import GlowBox from '../GlowBox.svelte';
  import Indicator, {Recency} from './Indicator.svelte';
  import Settings from '../settings/Settings.svelte';
  import Content from './Content.svelte';
  export let repo
  let showSettings = false
  let items = []
  let currentTab = 'all'
</script>

<article transition:fade="{{duration: 500}}" class="card vertical-flex">
  <header class="card-header">
    <div class="card-header-title">
      <p class="grow">{repo.title}</p>
      <a href="#" on:click|preventDefault={() => showSettings = !showSettings}>
        <i class="icon ion-md-settings" data-testid="settings" />
      </a>
    </div>
  </header>

  <div class="card-content grow">
    {#if showSettings }
      <Settings repoId={repo.id} on:repo-deleted/>
    {:else}
      <div class="content stack">
        <div class="tabs is-boxed">
          <ul>
            <li class:is-active={currentTab === 'all'}>
              <a on:click|preventDefault={() => currentTab = 'all'}>
                <span>All</span>
              </a>
            </li>
            <li class:is-active={currentTab === 'prs'}>
              <a on:click|preventDefault={() => currentTab = 'prs'}>
                <Github icon='git-pull-request' />
                <span>PRs</span>
              </a>
            </li>
            <li class:is-active={currentTab === 'issues'}>
              <a on:click|preventDefault={() => currentTab = 'issues'}>
                <Github icon='issue-opened' />
                <span>Issues</span>
              </a>
            </li>
          </ul>
        </div>
        {#if currentTab === 'all'}
          <Content items={[...repo.activity.prs, ...repo.activity.issues]} />
        {:else if currentTab === 'prs'}
          <Content items={[...repo.activity.prs]} />
        {:else if currentTab === 'issues'}
          <Content items={[...repo.activity.issues]} />
        {/if}

      </div>
    {/if}
  </div>
  <footer class="card-footer">
    <p class="is-size-7 card-footer-item">Last update 2min ago</p>
  </footer>
</article>

<style>
  ul {
    margin-left: 0;
  }
</style>
