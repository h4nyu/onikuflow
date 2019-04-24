import style from './style.css?module';

export default {
  name: 'SelectItem',
  props: {
    isSelected: Boolean,
  },
  computed: {
    barStyle() {
      if (this.isSelected) {
        return 'has-background-primary';
      }
      return null;
    },
  },
  render() {
    return (
      <div class={[style.layout]}>
        <div class={[style.bar, this.barStyle]}></div>
        <div class={[style.content]}>
          {this.$slots.default}
        </div>
      </div>
    );
  },
};