mkvm
====

Makes virtual machines on KVM.

http://github.com/frgomes/mkvm

Status
------
Inception

Motivation
----------

We are a group of people investing a lot of time and effort on Redhat OpenShift. As part of this effort and as part of a lot of tests we've already performed, we destroyed and rebuilt our custer literally dozens of times. I knew beforehand that such thing would happen and so, I've put the cart before the horses and I've built a very simple shell script intended to help us create virtual machines easily and quickly.

Along the way, we found that network settings should be more flexible than the shell script initially permitted. More flexibility was desirable for other aspects too, including the Linux distribution employed; initially only Centos 7.5 was supported but along the way we've added support for RHEL 7 and also support for Debian Stretch. The point is: it became clear that shell scripting is very limiting as complexity increases and as number of functionalities increase. Shell scripting is an easy and fast win in the short run but infeasible as complexity and functionalities increase.

Why Rust?

Since my intent is not starting a flame war here, let's simply accept that this is a codebase written in Rust. If you are convinced that Rust is for you, I invite you to contribute with PRs. Even if you think that Rust is not for you, you can still contribute with your time testing the results of our work or simply sharing with us your ideas and suggestions.

Code of conduct
---------------

Just be nice and we will be all fine.


For the impatient
-----------------

Out intent here is demonstrate how a virtual machine can be first defined and later effectively created.

    >>> This project is still in inception phase and this functionality does not work yet. 

First describe the characteristics of a virtual machine:

.. code-block:: yaml

  apiVersion: v1
  kind: MachineDescription
  description: A control center intended for cluster management
  name: okd10cc
  image: centos7.5
  vcpus: 1
  memory: 4GiB
  disks:
  - 25GiB
  - 20GiB
  network:
    domains:
    - okd10.mathminds.io
    nameservers:
    - ns1.he.net
    - ns2.he.net
    - ns3.he.net
    - ns4.he.net
    - ns5.he.net
    interfaces:
      eth0:
        mac: 52:54:00:2c:ca:ef
        ipv4: dhcp
        ipv6: dhcp
    ddns:
      strategy: ddns-update
      provider: he.net
      password: 453646433
  users:
    - rgomes:
      gecos: Richard Gomes
      groups: [
        - users
        - wheel
      shell: /bin/bash
      sudo: ALL=(ALL) NOPASSWD:ALL
      locked: true
      ssh-authorized-keys:
      - ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQDqVPeut7VFVHM7TDNMa9FmXGCEbJfU2AzG4f/bcSrJ6FhzhPhOwNF1IdrsFZ/5GvQWrO3PARuotllOsEkjhPBtlG9VKkxUmZqGkc8LkllqMeS7dqhEOnRJYG26kA4RGBnMhM+V4dJ9VgRB67CFQyh25xHU2O+8GUCSD8YaWNwApFdxGWO/AT9y1NKn+X9Qk+TdPnlCO62G1FZKIWsEr53pnQAwr4zNIHJcd2kp3eFP12n0VH71iNbjNL82tkOMqYxMQnpDOSstEWUDNy2z8Mx9HvE6ks60GvfQKrL4xduHkrVFG/wMlfkuH91l0Ey3200jRG4h1fIkpFo/9mtbhxWcQ5AnbFA1bjcoqks+tTQaXd1ImOB6iNmi5gmqeQQ7MPoUm+2EfkPANLi2/K+lJMutg+P7EeXftVPdiw1vd3octGMm9prBB3VnhayUIKvrxBCNSgzgxvlEfHg5sw4wldoZZMcNQ3NJV4LQju2y/bNIreB64CxMmk9CvTeRfb21Y4TwsSaPrABMUGJase/s/ZTgISHbx7KyqkPI1JF2LLTcmxWaFQxeyL/cNXmGz+LUmBgSf6IayUL1kMpfnVwKHqwuBQ3Hak0U0lxpjSqxx2SP8uR31ZNNsQCBocAcgwZ/MnJW8ZlGNs194PUwwPsBIwzYkJ6cmaSPP8cxgveyZqP+qw== rgomes@example.com

Then parse the description and store into the database:

.. code-block:: bash

  $ mkvm define vm okd10cc.yaml

Now you can create the virtual machine:

.. code-block:: bash

  $ mkvm create okd10cc

Building mkvm
-------------

Install Rust:

.. code-block:: bash

  $ wget https://raw.githubusercontent.com/frgomes/bash-scripts/master/user-install/install-rust.sh -O - | /bin/bash

Then simply build the application with ``cargo``:

.. code-block:: bash

  $ cargo build

If you are not acquainted to Rust
---------------------------------

IMHO, `Rust <http://rust-lang.org>`_ is the most relevant thing that happened in the domain of programming languages since C was invented back in the 60's. It's an innovative and unique programming language, which not only promisses memory safety, but in fact really allows you to write an operating system without facing segmentation faults and without using *gdb* at all (`see video here <https://www.youtube.com/watch?v=lBQHrj6vwAo>`_ and skip to 35:35mins until 38:30).

This video below is key for understanding the language. It's a video made by professioanal developers for experienced professional developers, going straight to the point, with useful and real life examples:

 * `Stanford Seminar - The Rust Programming Language <https://www.youtube.com/watch?v=O5vzLKg7y-k>`_

An excellent series of hands-on, introductory videos on Rust is here:

 * `Hello Rust, by Mattias Endler <https://hello-rust.show>`_

I use Emacs as an IDE for development in Rust:

 * `This my Emacs configuration folder. <https://github.com/frgomes/.emacs.d>`_
