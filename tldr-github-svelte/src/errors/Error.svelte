<script>
    import {clear, error} from './errorStore.js';
    import {fade} from 'svelte/transition';
    import GithubIcon from "../atoms/GithubIcon.svelte";

    let message = false;
    error.subscribe(value => {
        message = value.msg
    });
</script>

<div class="toast-container">
    {#if message }
        <article transition:fade="{{duration: 500}}" class="bg-orange-100 border-l-4 border-orange-500 text-orange-700 p-4">
            <p class="font-bold">Warning</p>
            <button on:click={clear}>
                <GithubIcon icon="x" />
            </button>
            <p>{message}</p>
        </article>
    {/if}
</div>

<style>
    .toast-container {
        position: absolute;
        top: 1rem;
        right: 1rem;
        z-index: 100;
    }
</style>
