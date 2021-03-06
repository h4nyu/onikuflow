import React from 'react';
import moment from 'moment';
import { storiesOf } from '@storybook/react';
import DatetimeInput from '~/components/DatetimeInput';


storiesOf('DatetimeInput', module)
  .add('default', () => {
    const date = moment();

    return (
      <DatetimeInput
        value={date}
        onChange={console.debug}
      />
    );
  });

