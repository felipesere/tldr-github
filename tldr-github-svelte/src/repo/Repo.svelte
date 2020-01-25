<script>
    import {fade} from 'svelte/transition';
    import Github from './Github.svelte';
    import Settings from '../settings/Settings.svelte';
    import TrackedItems from './TrackedItems.svelte';

    export let repo;
    let showSettings = false;

    let currentTab = 'all';

    function filterItems(theRepo, tab) {
        if (tab === 'all') {
            return [...theRepo.activity.prs, ...theRepo.activity.issues]
        }

        if (tab === 'prs') {
            return [...theRepo.activity.prs]
        }

        if (tab === 'issues') {
            return [...theRepo.activity.issues]
        }
    }

    $: items = filterItems(repo, currentTab);

    const tabs = [
        {value: 'all', text: 'All', icon: false},
        {value: 'prs', text: 'PRs', icon: 'git-pull-request'},
        {value: 'issues', text: 'Issues', icon: 'issue-opened'},
    ]
</script>

<article transition:fade="{{duration: 500}}" class="card vertical-flex">
    <header class="card-header">
        <div class="card-header-title">
            <p class="grow">{repo.title}</p>
            <a class="subtle" href="#" on:click|preventDefault={() => showSettings = !showSettings}>
                <i class="icon ion-md-settings" data-testid="settings"></i>
            </a>
        </div>
    </header>

    <div class="card-content grow">
        {#if showSettings }
            <Settings repo={repo} on:repo-deleted/>
        {:else}
            <div class="content stack">
                <div class="tabs is-boxed">
                    <ul>
                        {#each tabs as tab}
                            <li class:is-active={currentTab === tab.value}>
                                <a on:click|preventDefault={() => currentTab = tab.value }>
                                    <Github icon={tab.icon}/>
                                    <span>{tab.text}</span>
                                </a>
                            </li>
                        {/each}
                    </ul>
                </div>
                {#if (repo.activity.issues.length + repo.activity.prs.length) === 0 }
                    <p class="text-center subtle">No items are being tracked...</p>
                {:else}
                    <TrackedItems items={items}/>
                {/if}
            </div>
        {/if}
    </div>
</article>

<style>
    ul {
        margin-left: 0;
    }

    .card-content {
        height: 370px;
    }

    .subtle {
        color: #A0AEC0;

    }
</style>
