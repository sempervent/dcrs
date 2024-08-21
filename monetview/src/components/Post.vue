<template>
  <div class="post">
    <h1>Create a New Post</h1>
    <quill-editor v-model="content" ref="quillEditor"></quill-editor>

    <div class="upload-section">
      <input type="file" multiple @change="handleFileUpload" />
    </div>

    <button @click="submitPost">Submit Post</button>
  </div>
</template>

<script>
import { quillEditor } from 'vue-quill-editor';
import axios from 'axios';

export default {
  components: {
    quillEditor
  },
  data() {
    return {
      content: '',
      files: []
    };
  },
  methods: {
    handleFileUpload(event) {
      this.files = event.target.files;
    },
    submitPost() {
      const formData = new FormData();
      formData.append('content', this.content);

      for (let i = 0; i < this.files.length; i++) {
        formData.append('files[]', this.files[i]);
      }

      axios.post('/api/posts', formData)
        .then(response => {
          this.$router.push('/');
        })
        .catch(error => {
          console.error('Error submitting post:', error);
        });
    }
  }
};
</script>

<style>
.upload-section {
  margin-top: 20px;
}
</style>
