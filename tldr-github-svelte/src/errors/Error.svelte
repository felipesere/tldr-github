<script>
    import {clear, error} from './errorStore.js';
    import {fade} from 'svelte/transition';

    let message = false;
    error.subscribe(value => {
        message = value.msg
    });

    $: console.log($error)
</script>

<div class="toast-container">
    {#if message }
        <article transition:fade="{{duration: 500}}" class="message is-warning">
            <div class="message-header">
                <p>Warning</p>
                <button class="delete" aria-label="delete" on:click={clear}></button>
            </div>
            <div class="message-body">
                {message}
            </div>
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
