<!-- // 组件的显示隐藏请使用v-if而不要使用v-show，否则会出现意想不到的bug -->
<template>
    <div>
        <slot></slot>
        <slot name="bottom">
            <div style="text-align:center;">加载中...</div>
        </slot>
    </div>
</template>
<script>
export default {
    props: {
        parentElement: {
            type: Boolean,
            default: true
        },
        infiniteLoading: {
            type: Boolean,
            default: false
        },
        infiniteFunction: {
            type: Function
        }
    },
    data() {
        return {
            clientHeight: 0,
            bodyHeight: 0,
            parentHeight: 0,
            scrollTop: 0,
            clientTop: 0
        }
    },
    mounted() {
        let that = this;

        // 父元素文档高度、滚动条高度、可视区高度、组件元素高度
        // parentHeight、scrollTop、clientHeight、elementHeight
        if (this.parentElement) {
            let clientHeight = window.innerHeight;
            let scrollTop = Math.max(document.documentElement.scrollTop || document.body.scrollTop);
            let elementHeight = this.$el.getBoundingClientRect().height;
            let parentHeight = Math.max(document.documentElement.scrollHeight || document.body.scrollHeight);


            let timer = setInterval(() => {
                if (this.infiniteLoading) {
                    return false;
                }
                this.infiniteFunction();

                clientHeight = window.innerHeight;
                scrollTop = Math.max(document.documentElement.scrollTop || document.body.scrollTop);
                elementHeight = this.$el.getBoundingClientRect().height;

                if (clientHeight < elementHeight) {
                    clearInterval(timer);
                    timer = null;
                }
            }, 500);

            window.onscroll = function(){
                if (that.infiniteLoading) {
                    return false;
                }

                clientHeight = window.innerHeight;
                scrollTop = Math.max(document.documentElement.scrollTop || document.body.scrollTop);
                elementHeight = that.$el.getBoundingClientRect().height;

                parentHeight = Math.max(document.documentElement.scrollHeight || document.body.scrollHeight);

                if (parentHeight - 10 < (scrollTop + clientHeight)) {
                    that.infiniteFunction();
                }
            };
        } else {
            let parentNode = this.$el.parentNode;

            let parentHeight = parentNode.getBoundingClientRect().height;
            let scrollTop = parentNode.scrollTop;
            let elementHeight = this.$el.getBoundingClientRect().height;
            let clientHeight = parentNode.getBoundingClientRect().height;

            let timer = setInterval(() => {
                if (this.infiniteLoading) {
                    return false;
                }
                this.infiniteFunction();

                clientHeight = parentNode.getBoundingClientRect().height;
                elementHeight = this.$el.getBoundingClientRect().height;

                if (clientHeight < elementHeight) {
                    clearInterval(timer);
                    timer = null;
                }
            }, 500);

            parentNode.onscroll = function() {
                if (that.infiniteLoading) {
                    return false;
                }

                scrollTop = parentNode.scrollTop;
                parentHeight = parentNode.scrollHeight;
                clientHeight = parentNode.getBoundingClientRect().height;

                if (parentHeight - 10 < (scrollTop + clientHeight)) {
                    that.infiniteFunction();
                }
            };
        }

    },
    methods: {


    }
}
</script>
<style scoped>
</style>
    