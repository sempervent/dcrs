<template>
  <div class="post">
    <h1>Create a New Post</h1>
    <quill-editor v-model="content" ref="quillEditor"></quill-editor>

    <div class="upload-section">
      <input type="file" multiple @change="handleFileUpload" />
    </div>

    <div class="label-section">
      <h3>Select Labels</h3>
      <div v-for="label in predefinedLabels" :key="label.id">
        <input type="checkbox" :id="'label-' + label.id" :value="label.name" v-model="selectedLabels">
        <label :for="'label-' + label.id">{{ label.name }}</label>
      </div>
      <input type="text" v-model="newLabel" placeholder="Add custom label" @keydown.enter="addCustomLabel" />
      <button @click="addCustomLabel">Add Label</button>
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
      files: [],
      predefinedLabels: [],
      selectedLabels: [],
      newLabel: ''
    };
  },
  created() {
    // Fetch predefined labels from the backend
    axios.get('/api/labels/').then(response => {
      this.predefinedLabels = response.data;
    });
  },
  methods: {
    handleFileUpload(event) {
      this.files = event.target.files;
    },
    addCustomLabel() {
      if (this.newLabel && !this.selectedLabels.includes(this.newLabel)) {
        this.selectedLabels.push(this.newLabel);
        this.newLabel = '';
      }
    },
    submitPost() {
      const formData = new FormData();
      formData.append('content', this.content);

      for (let i = 0; i < this.files.length; i++) {
        formData.append('files[]', this.files[i]);
      }

      axios.post('/api/posts', formData)
        .then(response => {
          const postId = response.data.id;
          axios.post(`/api/contents/${postId}/labels/`, this.selectedLabels)
            .then(() => {
              this.$router.push('/');
            });
        })
        .catch(error => {
          console.error('Error submitting post:', error);
        });
    }
  }
};
</script>

<style>
.upload-section, .label-section {
  margin-top: 20px;
}
</style>
