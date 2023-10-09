<script lang="ts" setup>
import Alert from '@/components/Alert.vue';
import MyButton from '@/components/MyButton.vue';
import MyInput from '@/components/MyInput.vue';
import Input from '@/components/MyInput.vue';
import MyTextarea from '@/components/MyTextarea.vue';
import { useStore } from '@/stores';
import axios, { formToJSON } from 'axios';
import { reactive, ref, transformVNodeArgs } from 'vue';
import { useRouter } from 'vue-router';

const username = ref('')
const password = ref('')
const error = ref(false)

const store = useStore()
const router = useRouter()

const form = reactive({
    text: '',
    author: '',
    tags: ''
})

const submit = () => {
    const tags_array = form.tags.split(',').map(m => m.trim())

    axios.post('/quotes', {
        quote: form.text,
        tags: tags_array,
        author_name: form.author
    }).then((res) => {
        alert('Quote created')
        router.push('/my-quotes')
    }).catch(err => {
        alert('Error while creating quotes')
    });
}
</script>
<template>
    <main>
        <div class="container mx-auto">
            <div class="max-w-[700px] mx-auto">
                <div class="page-title mb-5">
                    <h1 class="text-4xl font-bold mb-8">Create Quote</h1>
                </div>
                <Alert type="danger" class="mb-5" v-if="error">{{ error }}</Alert>

                <form action="" @submit.prevent="submit">
                    <div class="mb-3">
                        <MyTextarea v-model="form.text" label="Text" name="text"></MyTextarea>
                    </div>
                    <div class="mb-3">
                        <MyInput v-model="form.author" label="Author" name="author" placeholder="Bill Gates"></MyInput>
                    </div>
                    <div class="mb-3">
                        <MyInput v-model="form.tags" label="Tags (separated by comma)" name="tag" placeholder="motivation,learning"></MyInput>
                    </div>
                    <div class="flex justify-end">
                        <MyButton>Submit</MyButton>
                    </div>
                </form>
            </div>
        </div>
    </main>
</template>