<script>
    import {createEventDispatcher} from 'svelte';
    import AddRepo from "./AddRepo.svelte";
    import Modal from "../atoms/Modal.svelte";

    const dispatch = createEventDispatcher();
    const close = () => dispatch('close');

    const keydown = ({keyCode}) => {
        // ESC key
        if (keyCode === 27) {
            close();
        }
    }
</script>

<svelte:window on:keydown={keydown}/>

<Modal>
    <p slot="title" class="modal-card-title">Add a new repo</p>
    <button slot="title" class="delete" aria-label="close" on:click={close}></button>

    <span slot="body">
        <AddRepo on:new-repo-added/>
    </span>

    <button slot="footer" class="btn-normal" on:click={close}>Done</button>
</Modal>

