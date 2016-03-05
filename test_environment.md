# hiding /etc/nsswitch.conf

libc always looks for configuration to /etc/nsswitch.conf
and it cannot be changed by any env variable or similar.

For testing purposes private mount namespace can be utilized
to maintain private version of the configuration.

This can be done by following commands:

# create new mount namespace and run bash in it
unshare -m bash

# new namespace inherits sharing level from original
# so we need to make it private to stop propagation
# of events, this should not make mount point in
# parent namespace private too
mount --make-private /
# $TEST_NSS_CONF is path to test configuration file
mount --bind $TEST_NSS_CONF /etc/nsswitch.conf