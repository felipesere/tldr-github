<script>
    import Error from './errors/Error.svelte';
    import Repo from './repo/Repo.svelte';
    import AddRepo from './AddRepo.svelte';
    import {onMount} from 'svelte';
    import SideMenu from "./menu/SideMenu.svelte";

    let repos = []

    const fetchRepos = async () => {
        const response = await fetch('/api/repos');
        repos = await response.json();
    }

    onMount(fetchRepos)
</script>

<div class="main">
    <SideMenu/>
    <!--
    <section class="section">
        <div class="container">
            <Error />
            <h1 class="title">
                Welcome to TLDR Github
            </h1>
            <p class="subtitle">
                These are the repos you are currently tracking
            </p>
        </div>
    </section>
    <section class="section">
        <div class="container">
            <AddRepo on:new-repo-added={fetchRepos}/>
        </div>
    </section>
    -->
    <div class="container my-4">
        {#if repos.length === 0}
            <p>No repos added yet</p>
        {/if}
        <div class="grid">
            {#each repos as repo (repo.id) }
                <Repo repo={repo}  on:repo-deleted={fetchRepos}/>
            {/each}
        </div>
    </div>
</div>

<style>
    .main {
        display: flex;
    }

    .my-4 {
        margin-top: 3rem;
        margin-bottom: 3rem;
    }
</style>
