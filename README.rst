mkvm
====

Makes virtual machines on Libvirt. Implemented in Rust.

http://github.com/frgomes/mkvm

Status
------
Under heavy development

Motivation
----------

We would like to generate configuration files to be consumed by `virsh <https://www.libvirt.org/manpages/virsh.html>`_ so that we could automate the entire process of deployment of a cluster. The solution needs to be small and simple yet, needs to be flexible, powerful, robust and reliable. We should be able to integrate the solution later into bigger system so that the user chooses which flavor of cluster is needed, adjusts parameters and preferences, and after 5 minutes the cluster is ready to go. Even thought there are several tools intended to cover alll requirements mentioned here or not mentioned here, there's no single tool which attends all requirements and, at the same time, is sufficiently simple and manageable.

Requirements
------------

A non-exaustive list of requirements is presented below:

 - template oriented;
 - generate .xml files for virsh;
 - variables are specified and substituted in templates according to `Golang template conventions <https://golang.org/pkg/text/template/>`_;
 - IPv6 must be necessarily enabled in all virtual machines;
 - publish hostnames ane their global IPv6 addresses onto a public DNS;
   
For the impatient
-----------------

Password for root is: root

Nothing else here. Stay tuned.
