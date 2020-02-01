<script>
    import {fly, fade} from 'svelte/transition'
    import {createEventDispatcher} from 'svelte';
    import AddRepo from "./AddRepo.svelte";

    const dispatch = createEventDispatcher();
    const close = () => dispatch('close');

    const keydown = ({keyCode}) => {
        // ESC key
        if (keyCode === 27) {
            close();
        }
    }
</script>

<svelte:window on:keydown={keydown} />

<div class="modal is-active">
    <div class="modal-background" transition:fade></div>
    <div in:fly={{ y: 50, duration: 300}} out:fly={{ y: -50, duration: 300}} class="modal-card">
        <header class="modal-card-head">
            <p class="modal-card-title">Add a new repo</p>
            <button class="delete" aria-label="close" on:click={close}></button>
        </header>
        <section class="modal-card-body">
            <AddRepo on:new-repo-added />
        </section>
        <footer class="modal-card-foot">
            <button class="button" on:click={close}>Done</button>
        </footer>
    </div>
</div>

<style>
    .modal-background {
        opacity: 0.5;
    }
</style>