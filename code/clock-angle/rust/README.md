# Clock Angle

Calculate the angle between the two arms of a clock.

## Solution

The clock's arms rotate around the dial, with the hour
hand moving between 0 and 12 hours, and the minute hand
moving between 0 and 60 minutes. Given the hour or the
minute hand, one can calculate the angle that arm and the
zero position.

It can be made much easier by thinking about the hand's travel
around the clock in terms of percentages, and that 100% around
the clock is 360 degrees. An hour hand at the 6 position is
at `6/12=0.5 => 0.5*360=180 degrees`. Similarly, the minute hand
at 20 minutes is at `20/60=1/3 => (1/3)*360=120 degrees`.

Once we have these two angles, we can simply take the difference
between them, and take the absolute value of that difference.

There will need to be logic to handle parsing out the time as well,
but we'll put that outside the calculation function.
