import { createMachine } from '../rust/pkg';
describe('@xstate/fsm', () => {
  it('Runs val', () => {
    expect(createMachine({}, {})).toBe(undefined);
  });
});
