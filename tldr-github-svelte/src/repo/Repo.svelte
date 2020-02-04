<script>
    import {fade} from 'svelte/transition';
    import Github from '../atoms/GithubIcon.svelte';
    import Settings from '../settings/Settings.svelte';
    import TrackedItems from './TrackedItems.svelte';
    import GithubIcon from "../atoms/GithubIcon.svelte";

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

<article transition:fade="{{duration: 500}}" class="border border-gray-300 shadow-md max-w-full flex flex-col">
    <header class="shadow bg-gray-200 border-gray-400 border-b-2">
        <div class="py-3 px-6 flex flex-grow font-bold">
            <p class="flex-grow text-gray-700 leading-loose">{repo.title}</p>
            <a class="text-gray-600 fill-current" data-testid="settings" href="#"
               on:click|preventDefault={() => showSettings = !showSettings}>
                <GithubIcon icon="gear"/>
            </a>
        </div>
    </header>

    <div class="p-6 flex-grow card-content">
        {#if showSettings }
            <Settings repo={repo} on:repo-deleted on:repo-updated/>
        {:else}
            <div class="stack">
                <ul class="flex border-b list-none">
                    {#each tabs as tab (tab.value)}
                        <li class:active={tab.value === currentTab} class:inactive={tab.value !== currentTab}>
                            <a class="cursor-pointer" on:click|preventDefault={() => currentTab = tab.value }>
                                <Github icon={tab.icon}/>
                                <span>{tab.text}</span>
                            </a>
                        </li>
                    {/each}
                </ul>
                {#if (repo.activity.issues.length + repo.activity.prs.length) === 0 }
                    <p class="text-center text-gray-600">No items are being tracked...</p>
                {:else}
                    <TrackedItems items={items}/>
                {/if}
            </div>
        {/if}
    </div>
</article>

<style>
    .active {
        @apply bg-white inline-block border-l border-t border-r rounded-t py-2 px-4 text-blue-700 font-semibold -mb-px font-light
    }

    .inactive {
        @apply bg-white inline-block py-2 px-4 text-blue-500 font-semibold font-light
    }
    .inactive:hover {
        @apply text-blue-800
    }

    .card-content {
        height: 370px;
    }
</style>
