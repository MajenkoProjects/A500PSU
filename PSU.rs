ha:cschem-sheet-v1 {
	ha:obj_indirect.1 {
		li:objects {
		}
	}
	ha:obj_direct.2 {
		uuid=aXxUDjrO1NamlIZ+2vIAAAAC;
		li:objects {
			ha:pen.sheet-decor { shape=round; size=125; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.sheet-decor-fill { shape=round; size=125; color=#bbbbbb; font_height=3000; font_family=sans; }
			ha:pen.titlebox-frame { shape=round; size=250; color=#777777; font_height=0; }
			ha:pen.titlebox-fill { shape=round; size=250; color=#bbffbb; font_height=0; }
			ha:pen.titlebox-big { shape=round; size=250; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.titlebox-small { shape=round; size=250; color=#777777; font_height=1500; font_family=sans; }
			ha:pen.wire { shape=round; size=250; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.bus { shape=round; size=1500; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.hub { shape=round; size=3000; color=#6666ff; font_height=3000; font_family=sans; }
			ha:pen.sym-decor { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; }
			ha:pen.sym-decor-fill { shape=round; size=125; color=#99ff99; font_height=3000; font_family=sans; }
			ha:pen.sym-primary { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.sym-secondary { shape=round; size=125; color=#33bb33; font_height=3000; font_family=sans; }
			ha:pen.term-decor { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.term-primary { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.term-secondary { shape=round; size=250; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.busterm-decor { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.busterm-primary { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.busterm-secondary { shape=round; size=1500; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.junction { shape=round; size=1000; color=#2222bb; font_height=3000; font_family=sans; }
			ha:group.1 {
				uuid=8/SYZ/pfBzxyqFA12WQAAAAC; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAg;
				li:objects {
					ha:polygon.11 {
						li:outline {
							ha:line { x1=0; y1=0; x2=80000; y2=0; }
							ha:line { x1=80000; y1=0; x2=80000; y2=20000; }
							ha:line { x1=80000; y1=20000; x2=0; y2=20000; }
							ha:line { x1=0; y1=20000; x2=0; y2=0; }
						}
						stroke=titlebox-frame;
						fill=titlebox-fill;
					}
					ha:line.12 { x1=0; y1=10000; x2=80000; y2=10000; stroke=titlebox-frame; }
					ha:line.13 { x1=40000; y1=10000; x2=40000; y2=0; stroke=titlebox-frame; }
					ha:text.20 { x1=1000; y1=16500; dyntext=0; stroke=titlebox-big; text=TITLE; }
					ha:text.21 { x1=1000; y1=10500; x2=79000; y2=16000; dyntext=1; stroke=titlebox-big; text=%../../A.title%; }
					ha:text.22 { x1=1000; y1=5500; dyntext=0; stroke=titlebox-small; text={PROJECT:}; }
					ha:text.23 { x1=13000; y1=5500; x2=39000; y2=9500; dyntext=1; stroke=titlebox-big; text=%project.name%; }
					ha:text.24 { x1=1000; y1=500; dyntext=0; stroke=titlebox-small; text={PAGE:}; }
					ha:text.25 { x1=10000; y1=500; x2=39000; y2=4500; dyntext=1; stroke=titlebox-big; text=%../../A.page%; }
					ha:text.26 { x1=41000; y1=5500; dyntext=0; stroke=titlebox-small; text={FILE:}; }
					ha:text.27 { x1=48000; y1=5500; x2=79000; y2=9500; dyntext=1; stroke=titlebox-big; text=%filename%; }
					ha:text.28 { x1=41000; y1=500; dyntext=0; stroke=titlebox-small; text={MAINTAINER:}; }
					ha:text.29 { x1=55000; y1=500; x2=79000; y2=4500; dyntext=1; stroke=titlebox-big; text=%../../A.maintainer%; }
					ha:text.30 { x1=79000; y1=16000; mirx=1; dyntext=1; stroke=sheet-decor; text=%stance.model% %stance.sub_major% %stance.sub_minor% %stance.test_bench% %view.name%; }
				}
				ha:attrib {
					purpose=titlebox
				}
			}
			ha:group.16 {
				uuid=aXxUDjrO1NamlIZ+2vIAAAAK;
				x=108000; y=132000;
				li:objects {
					ha:group.1 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAAL; src_uuid=aXxUDjrO1NamlIZ+2vIAAAAD;
						x=-12000; y=12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=VIN
							role=terminal
						}
					}
					ha:group.2 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAAM; src_uuid=aXxUDjrO1NamlIZ+2vIAAAAE;
						x=-12000; y=8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=EN
							role=terminal
						}
					}
					ha:group.3 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAAN; src_uuid=aXxUDjrO1NamlIZ+2vIAAAAF;
						x=-12000; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=SS
							role=terminal
						}
					}
					ha:group.4 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAAO; src_uuid=aXxUDjrO1NamlIZ+2vIAAAAG;
						x=12000; y=12000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=VOUT
							role=terminal
						}
					}
					ha:group.5 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAAP; src_uuid=aXxUDjrO1NamlIZ+2vIAAAAH;
						x=12000; y=8000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=FB
							role=terminal
						}
					}
					ha:group.6 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAAQ; src_uuid=aXxUDjrO1NamlIZ+2vIAAAAI;
						x=12000; y=4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=PG
							role=terminal
						}
					}
					ha:group.7 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAAR; src_uuid=aXxUDjrO1NamlIZ+2vIAAAAJ;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=GND
							role=terminal
						}
					}
					ha:polygon.8 {
						li:outline {
							ha:line { x1=-8000; y1=16000; x2=-8000; y2=0; }
							ha:line { x1=-8000; y1=0; x2=8000; y2=0; }
							ha:line { x1=8000; y1=0; x2=8000; y2=16000; }
							ha:line { x1=8000; y1=16000; x2=-8000; y2=16000; }
						}
						stroke=sym-decor;
						fill=sym-decor-fill;
					}
					ha:text.9 { x1=-7000; y1=10000; dyntext=0; stroke=sym-decor; text=VIN; }
					ha:text.10 { x1=-7000; y1=6000; dyntext=0; stroke=sym-decor; text=EN; }
					ha:text.11 { x1=-7000; y1=2000; dyntext=0; stroke=sym-decor; text=SS; }
					ha:text.12 { x1=7000; y1=10000; mirx=1; dyntext=0; stroke=sym-decor; text=VOUT; }
					ha:text.13 { x1=7000; y1=6000; mirx=1; dyntext=0; stroke=sym-decor; text=FB; }
					ha:text.14 { x1=7000; y1=2000; mirx=1; dyntext=0; stroke=sym-decor; text=PG; }
					ha:text.15 { x1=0; y1=16000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:text.16 { x1=-16000; y1=-4000; dyntext=1; stroke=sym-secondary; text=%../a.device%; floater=1; }
				}
				ha:attrib {
					device=TPS82130SIL
					footprint=TPS82130SILT
					name=U1
					li:portmap {
						{VIN->pcb/pinnum=2}
						{EN->pcb/pinnum=1}
						{SS->pcb/pinnum=8}
						{VOUT->pcb/pinnum=4 5}
						{FB->pcb/pinnum=6}
						{PG->pcb/pinnum=7}
						{GND->pcb/pinnum=3 EP}
					}
					role=symbol
				}
			}
			ha:group.24 {
				uuid=aXxUDjrO1NamlIZ+2vIAAAAx; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=108000; y=56000;
				li:objects {
					ha:group.1 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAAy; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							drc/require_graphical_conn=1
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.25 {
				uuid=aXxUDjrO1NamlIZ+2vIAAAAz; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=152000; y=156000;
				li:objects {
					ha:group.1 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAA0; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							drc/require_graphical_conn=1
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.26 {
				uuid=aXxUDjrO1NamlIZ+2vIAAAA1;
				x=0; y=-16000;
				li:objects {
					ha:line.1 { x1=108000; y1=72000; x2=108000; y2=80000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.30 {
				uuid=aXxUDjrO1NamlIZ+2vIAAAA8; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=84000; y=132000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAA9; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAA+; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=capacitor; prio=31050; }
					footprint=0603
					name=C3
					role=symbol
					ha:spice/prefix = { value=C; prio=31050; }
					value=220nF
				}
			}
			ha:group.31 {
				uuid=aXxUDjrO1NamlIZ+2vIAAAA/; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=84000; y=68000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=aXxUDjrO1NamlIZ+2vIAAABA; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=aXxUDjrO1NamlIZ+2vIAAABB; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=capacitor; prio=31050; }
					footprint=0603
					name=C4
					role=symbol
					ha:spice/prefix = { value=C; prio=31050; }
					value=220nF
				}
			}
			ha:group.32 {
				uuid=aXxUDjrO1NamlIZ+2vIAAABC;
				li:objects {
					ha:line.1 { x1=96000; y1=136000; x2=84000; y2=136000; stroke=wire; }
					ha:line.2 { x1=84000; y1=136000; x2=84000; y2=132000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.33 {
				li:conn {
					/2/32/1
					/2/16/3/1
				}
			}
			ha:group.35 {
				uuid=aXxUDjrO1NamlIZ+2vIAAABD;
				x=0; y=-16000;
				li:objects {
					ha:line.2 { x1=84000; y1=88000; x2=84000; y2=84000; stroke=wire; }
					ha:line.3 { x1=84000; y1=88000; x2=96000; y2=88000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.38 {
				uuid=aXxUDjrO1NamlIZ+2vIAAABG; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=84000; y=44000;
				li:objects {
					ha:group.1 {
						uuid=aXxUDjrO1NamlIZ+2vIAAABH; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							drc/require_graphical_conn=1
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.43 {
				uuid=aXxUDjrO1NamlIZ+2vIAAABL; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=84000; y=108000;
				li:objects {
					ha:group.1 {
						uuid=aXxUDjrO1NamlIZ+2vIAAABM; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							drc/require_graphical_conn=1
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:connection.46 {
				li:conn {
					/2/43/1/1
					/2/84/3
					/2/84/2
				}
			}
			ha:group.47 {
				uuid=aXxUDjrO1NamlIZ+2vIAAABO;
				li:objects {
					ha:line.3 { x1=144000; y1=144000; x2=144000; y2=144000; stroke=junction; }
					ha:line.5 { x1=144000; y1=160000; x2=152000; y2=160000; stroke=wire; }
					ha:line.6 { x1=144000; y1=144000; x2=144000; y2=160000; stroke=wire; }
					ha:line.7 { x1=152000; y1=160000; x2=152000; y2=156000; stroke=wire; }
					ha:line.8 { x1=152000; y1=140000; x2=152000; y2=144000; stroke=wire; }
					ha:line.9 { x1=152000; y1=144000; x2=152000; y2=144000; stroke=junction; }
					ha:line.10 { x1=120000; y1=144000; x2=176000; y2=144000; stroke=wire; }
					ha:line.11 { x1=176000; y1=144000; x2=176000; y2=132000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.48 {
				li:conn {
					/2/16/4/1
					/2/47/10
				}
			}
			ha:connection.50 {
				li:conn {
					/2/25/1/1
					/2/47/7
				}
			}
			ha:group.51 {
				uuid=aXxUDjrO1NamlIZ+2vIAAABy; src_uuid=aXxUDjrO1NamlIZ+2vIAAABv;
				x=8000; y=144000; mirx=1; miry=1;
				li:objects {
					ha:text.1 { x1=0; y1=-2000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=aXxUDjrO1NamlIZ+2vIAAABz; src_uuid=aXxUDjrO1NamlIZ+2vIAAABw;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAB0; src_uuid=aXxUDjrO1NamlIZ+2vIAAABx;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:polygon.4 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=6000; }
							ha:line { x1=0; y1=6000; x2=4000; y2=6000; }
							ha:line { x1=4000; y1=6000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
					ha:text.5 { x1=0; y1=-6000; dyntext=1; stroke=sym-secondary; text=%../A.device%; floater=1; }
				}
				ha:attrib {
					device=694106106102
					footprint=694106106102
					name=CONN1
					role=symbol
					spice/omit=yes
				}
			}
			ha:group.53 {
				uuid=aXxUDjrO1NamlIZ+2vIAAAB1;
				li:objects {
					ha:line.1 { x1=12000; y1=140000; x2=24000; y2=140000; stroke=wire; }
					ha:line.3 { x1=24000; y1=132000; x2=24000; y2=140000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.55 {
				uuid=aXxUDjrO1NamlIZ+2vIAAAB4; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=24000; y=132000;
				li:objects {
					ha:group.1 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAB5; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							drc/require_graphical_conn=1
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:connection.56 {
				li:conn {
					/2/53/3
					/2/55/1/1
				}
			}
			ha:group.59 {
				uuid=aXxUDjrO1NamlIZ+2vIAAACS; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=152000; y=76000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=aXxUDjrO1NamlIZ+2vIAAACT; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=aXxUDjrO1NamlIZ+2vIAAACU; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=resistor; prio=31050; }
					footprint=0603
					name=R3
					role=symbol
					ha:spice/prefix = { value=R; prio=31050; }
					value=523k
				}
			}
			ha:group.60 {
				uuid=aXxUDjrO1NamlIZ+2vIAAACV; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=152000; y=48000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=aXxUDjrO1NamlIZ+2vIAAACW; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=aXxUDjrO1NamlIZ+2vIAAACX; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=resistor; prio=31050; }
					footprint=0603
					name=R4
					role=symbol
					ha:spice/prefix = { value=R; prio=31050; }
					value=100k
				}
			}
			ha:group.61 {
				uuid=aXxUDjrO1NamlIZ+2vIAAACe; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=152000; y=140000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=aXxUDjrO1NamlIZ+2vIAAACf; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=aXxUDjrO1NamlIZ+2vIAAACg; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=resistor; prio=31050; }
					footprint=0603
					name=R1
					role=symbol
					ha:spice/prefix = { value=R; prio=31050; }
					value=140k
				}
			}
			ha:group.62 {
				uuid=aXxUDjrO1NamlIZ+2vIAAACh; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=152000; y=112000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=aXxUDjrO1NamlIZ+2vIAAACi; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=aXxUDjrO1NamlIZ+2vIAAACj; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=resistor; prio=31050; }
					footprint=0603
					name=R2
					role=symbol
					ha:spice/prefix = { value=R; prio=31050; }
					value=10k
				}
			}
			ha:group.68 {
				uuid=aXxUDjrO1NamlIZ+2vIAAACl;
				x=0; y=-16000;
				li:objects {
					ha:line.1 { x1=152000; y1=72000; x2=152000; y2=64000; stroke=wire; }
					ha:line.2 { x1=152000; y1=68000; x2=136000; y2=68000; stroke=wire; }
					ha:line.3 { x1=152000; y1=68000; x2=152000; y2=68000; stroke=junction; }
					ha:line.4 { x1=136000; y1=68000; x2=136000; y2=92000; stroke=wire; }
					ha:line.5 { x1=136000; y1=92000; x2=120000; y2=92000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.73 {
				uuid=aXxUDjrO1NamlIZ+2vIAAACp; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=152000; y=24000;
				li:objects {
					ha:group.1 {
						uuid=aXxUDjrO1NamlIZ+2vIAAACq; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							drc/require_graphical_conn=1
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.76 {
				uuid=aXxUDjrO1NamlIZ+2vIAAACu; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=64000; y=68000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=aXxUDjrO1NamlIZ+2vIAAACv; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=aXxUDjrO1NamlIZ+2vIAAACw; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=capacitor; prio=31050; }
					footprint=1206
					name=C2
					role=symbol
					ha:spice/prefix = { value=C; prio=31050; }
					value=10uF
				}
			}
			ha:group.77 {
				uuid=aXxUDjrO1NamlIZ+2vIAAACx; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=176000; y=68000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=aXxUDjrO1NamlIZ+2vIAAACy; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=aXxUDjrO1NamlIZ+2vIAAACz; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=capacitor; prio=31050; }
					footprint=1206
					name=C6
					role=symbol
					ha:spice/prefix = { value=C; prio=31050; }
					value=10uF
				}
			}
			ha:group.81 {
				uuid=aXxUDjrO1NamlIZ+2vIAAAC4; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=64000; y=132000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAC5; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAC6; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=capacitor; prio=31050; }
					footprint=1206
					name=C1
					role=symbol
					ha:spice/prefix = { value=C; prio=31050; }
					value=10uF
				}
			}
			ha:group.82 {
				uuid=aXxUDjrO1NamlIZ+2vIAAAC7; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=176000; y=132000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAC8; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAC9; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=capacitor; prio=31050; }
					footprint=1206
					name=C5
					role=symbol
					ha:spice/prefix = { value=C; prio=31050; }
					value=10uF
				}
			}
			ha:group.84 {
				uuid=aXxUDjrO1NamlIZ+2vIAAAC+;
				li:objects {
					ha:line.1 { x1=64000; y1=112000; x2=64000; y2=108000; stroke=wire; }
					ha:line.2 { x1=84000; y1=112000; x2=84000; y2=108000; stroke=wire; }
					ha:line.3 { x1=64000; y1=108000; x2=84000; y2=108000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.87 {
				uuid=aXxUDjrO1NamlIZ+2vIAAAC/;
				x=0; y=-16000;
				li:objects {
					ha:line.1 { x1=64000; y1=64000; x2=64000; y2=60000; stroke=wire; }
					ha:line.2 { x1=84000; y1=60000; x2=84000; y2=64000; stroke=wire; }
					ha:line.3 { x1=64000; y1=60000; x2=84000; y2=60000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.117 {
				li:conn {
					/2/26/1
					/2/24/1/1
				}
			}
			ha:connection.124 {
				li:conn {
					/2/59/2/1
					/2/185/3
				}
			}
			ha:connection.126 {
				li:conn {
					/2/68/1
					/2/60/2/1
				}
			}
			ha:connection.127 {
				li:conn {
					/2/68/1
					/2/59/1/1
				}
			}
			ha:connection.129 {
				li:conn {
					/2/60/1/1
					/2/162/2
				}
			}
			ha:connection.130 {
				li:conn {
					/2/73/1/1
					/2/162/2
					/2/162/3
				}
			}
			ha:connection.137 {
				li:conn {
					/2/87/2
					/2/38/1/1
					/2/87/3
				}
			}
			ha:connection.140 {
				li:conn {
					/2/16/1/1
					/2/188/4
				}
			}
			ha:connection.142 {
				li:conn {
					/2/16/2/1
					/2/188/6
				}
			}
			ha:connection.143 {
				li:conn {
					/2/47/8
					/2/61/2/1
				}
			}
			ha:connection.145 {
				li:conn {
					/2/16/7/1
					/2/152/3
				}
			}
			ha:group.152 {
				uuid=aXxUDjrO1NamlIZ+2vIAAADE;
				li:objects {
					ha:line.3 { x1=108000; y1=88000; x2=108000; y2=128000; stroke=wire; }
					ha:line.4 { x1=152000; y1=88000; x2=152000; y2=92000; stroke=wire; }
					ha:line.5 { x1=152000; y1=88000; x2=152000; y2=88000; stroke=junction; }
					ha:line.6 { x1=176000; y1=112000; x2=176000; y2=88000; stroke=wire; }
					ha:line.7 { x1=176000; y1=88000; x2=176000; y2=88000; stroke=junction; }
					ha:line.8 { x1=108000; y1=88000; x2=220000; y2=88000; stroke=wire; }
					ha:line.9 { x1=220000; y1=88000; x2=220000; y2=136000; stroke=wire; }
					ha:line.10 { x1=220000; y1=136000; x2=248000; y2=136000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.154 {
				li:conn {
					/2/62/1/1
					/2/152/4
				}
			}
			ha:group.155 {
				uuid=aXxUDjrO1NamlIZ+2vIAAADF;
				li:objects {
					ha:line.1 { x1=132000; y1=140000; x2=120000; y2=140000; stroke=wire; }
					ha:line.3 { x1=152000; y1=116000; x2=132000; y2=116000; stroke=wire; }
					ha:line.4 { x1=132000; y1=116000; x2=132000; y2=140000; stroke=wire; }
					ha:line.5 { x1=152000; y1=120000; x2=152000; y2=112000; stroke=wire; }
					ha:line.6 { x1=152000; y1=116000; x2=152000; y2=116000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.156 {
				li:conn {
					/2/155/1
					/2/16/5/1
				}
			}
			ha:connection.157 {
				li:conn {
					/2/155/5
					/2/61/1/1
				}
			}
			ha:connection.158 {
				li:conn {
					/2/155/5
					/2/62/2/1
				}
			}
			ha:group.162 {
				uuid=aXxUDjrO1NamlIZ+2vIAAADG;
				li:objects {
					ha:line.2 { x1=152000; y1=28000; x2=152000; y2=24000; stroke=wire; }
					ha:line.3 { x1=176000; y1=24000; x2=152000; y2=24000; stroke=wire; }
					ha:line.4 { x1=176000; y1=24000; x2=176000; y2=48000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.164 {
				uuid=aXxUDjrO1NamlIZ+2vIAAAEH; src_uuid=aXxUDjrO1NamlIZ+2vIAAAEC;
				x=252000; y=124000;
				li:objects {
					ha:text.1 { x1=0; y1=-2000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAEI; src_uuid=aXxUDjrO1NamlIZ+2vIAAAED;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAEJ; src_uuid=aXxUDjrO1NamlIZ+2vIAAAEE;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.4 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAEK; src_uuid=aXxUDjrO1NamlIZ+2vIAAAEF;
						x=0; y=8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=3
							role=terminal
						}
					}
					ha:group.5 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAEL; src_uuid=aXxUDjrO1NamlIZ+2vIAAAEG;
						x=0; y=12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=4
							role=terminal
						}
					}
					ha:polygon.6 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=14000; }
							ha:line { x1=0; y1=14000; x2=4000; y2=14000; }
							ha:line { x1=4000; y1=14000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint={connector(1, 4, spacing=200.0mil, pin_ringdia=2.0mm, pin_drill=1.2mm)}
					name=CONN2
					role=symbol
					spice/omit=yes
				}
			}
			ha:connection.165 {
				li:conn {
					/2/81/2/1
					/2/188/3
				}
			}
			ha:connection.166 {
				li:conn {
					/2/81/1/1
					/2/84/1
				}
			}
			ha:connection.167 {
				li:conn {
					/2/30/1/1
					/2/84/2
				}
			}
			ha:connection.169 {
				li:conn {
					/2/30/2/1
					/2/32/2
				}
			}
			ha:connection.170 {
				li:conn {
					/2/76/1/1
					/2/87/1
				}
			}
			ha:connection.171 {
				li:conn {
					/2/31/2/1
					/2/35/2
				}
			}
			ha:connection.172 {
				li:conn {
					/2/31/1/1
					/2/87/2
				}
			}
			ha:connection.173 {
				li:conn {
					/2/76/2/1
					/2/188/14
				}
			}
			ha:connection.174 {
				li:conn {
					/2/77/1/1
					/2/162/4
				}
			}
			ha:connection.175 {
				li:conn {
					/2/77/2/1
					/2/185/5
				}
			}
			ha:connection.176 {
				li:conn {
					/2/82/1/1
					/2/152/6
				}
			}
			ha:connection.177 {
				li:conn {
					/2/82/2/1
					/2/47/11
				}
			}
			ha:text.178 { x1=204000; y1=72000; dyntext=0; stroke=sheet-decor; text=+5V; }
			ha:text.179 { x1=204000; y1=92000; dyntext=0; stroke=sheet-decor; text=-12V; }
			ha:text.180 { x1=204000; y1=164000; dyntext=0; stroke=sheet-decor; text=+12V; }
			ha:group.181 {
				uuid=aXxUDjrO1NamlIZ+2vIAAAEO; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=244000; y=120000;
				li:objects {
					ha:group.1 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAEP; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							drc/require_graphical_conn=1
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.182 {
				uuid=aXxUDjrO1NamlIZ+2vIAAAEQ;
				li:objects {
					ha:line.1 { x1=248000; y1=124000; x2=244000; y2=124000; stroke=wire; }
					ha:line.2 { x1=244000; y1=124000; x2=244000; y2=120000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.183 {
				li:conn {
					/2/182/1
					/2/164/2/1
				}
			}
			ha:connection.184 {
				li:conn {
					/2/182/2
					/2/181/1/1
				}
			}
			ha:group.185 {
				uuid=aXxUDjrO1NamlIZ+2vIAAAER;
				li:objects {
					ha:line.1 { x1=248000; y1=128000; x2=228000; y2=128000; stroke=wire; }
					ha:line.2 { x1=228000; y1=128000; x2=228000; y2=80000; stroke=wire; }
					ha:line.3 { x1=152000; y1=80000; x2=152000; y2=76000; stroke=wire; }
					ha:line.4 { x1=152000; y1=80000; x2=152000; y2=80000; stroke=junction; }
					ha:line.5 { x1=176000; y1=80000; x2=176000; y2=68000; stroke=wire; }
					ha:line.6 { x1=176000; y1=80000; x2=176000; y2=80000; stroke=junction; }
					ha:line.8 { x1=120000; y1=80000; x2=228000; y2=80000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.186 {
				li:conn {
					/2/185/1
					/2/164/3/1
				}
			}
			ha:connection.187 {
				li:conn {
					/2/152/10
					/2/164/5/1
				}
			}
			ha:group.188 {
				uuid=aXxUDjrO1NamlIZ+2vIAAAES;
				li:objects {
					ha:line.1 { x1=248000; y1=132000; x2=228000; y2=132000; stroke=wire; }
					ha:line.2 { x1=228000; y1=132000; x2=228000; y2=172000; stroke=wire; }
					ha:line.3 { x1=64000; y1=144000; x2=64000; y2=132000; stroke=wire; }
					ha:line.4 { x1=12000; y1=144000; x2=96000; y2=144000; stroke=wire; }
					ha:line.5 { x1=84000; y1=140000; x2=84000; y2=144000; stroke=wire; }
					ha:line.6 { x1=96000; y1=140000; x2=84000; y2=140000; stroke=wire; }
					ha:line.7 { x1=64000; y1=144000; x2=64000; y2=144000; stroke=junction; }
					ha:line.8 { x1=84000; y1=144000; x2=84000; y2=144000; stroke=junction; }
					ha:line.9 { x1=36000; y1=144000; x2=36000; y2=144000; stroke=junction; }
					ha:line.10 { x1=84000; y1=76000; x2=96000; y2=76000; stroke=wire; }
					ha:line.11 { x1=84000; y1=76000; x2=84000; y2=80000; stroke=wire; }
					ha:line.12 { x1=84000; y1=80000; x2=84000; y2=80000; stroke=junction; }
					ha:line.13 { x1=36000; y1=80000; x2=96000; y2=80000; stroke=wire; }
					ha:line.14 { x1=64000; y1=80000; x2=64000; y2=68000; stroke=wire; }
					ha:line.15 { x1=64000; y1=80000; x2=64000; y2=80000; stroke=junction; }
					ha:line.16 { x1=36000; y1=80000; x2=36000; y2=144000; stroke=wire; }
					ha:line.17 { x1=28000; y1=144000; x2=28000; y2=172000; stroke=wire; }
					ha:line.18 { x1=28000; y1=144000; x2=28000; y2=144000; stroke=junction; }
					ha:line.20 { x1=28000; y1=172000; x2=228000; y2=172000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.189 {
				li:conn {
					/2/188/1
					/2/164/4/1
				}
			}
			ha:group.190 {
				uuid=aXxUDjrO1NamlIZ+2vIAAAEb; src_uuid=aXxUDjrO1NamlIZ+2vIAAAAK;
				x=108000; y=68000;
				li:objects {
					ha:group.1 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAEc; src_uuid=aXxUDjrO1NamlIZ+2vIAAAAD;
						x=-12000; y=12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=VIN
							role=terminal
						}
					}
					ha:group.2 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAEd; src_uuid=aXxUDjrO1NamlIZ+2vIAAAAE;
						x=-12000; y=8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=EN
							role=terminal
						}
					}
					ha:group.3 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAEe; src_uuid=aXxUDjrO1NamlIZ+2vIAAAAF;
						x=-12000; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=SS
							role=terminal
						}
					}
					ha:group.4 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAEf; src_uuid=aXxUDjrO1NamlIZ+2vIAAAAG;
						x=12000; y=12000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=VOUT
							role=terminal
						}
					}
					ha:group.5 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAEg; src_uuid=aXxUDjrO1NamlIZ+2vIAAAAH;
						x=12000; y=8000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=FB
							role=terminal
						}
					}
					ha:group.6 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAEh; src_uuid=aXxUDjrO1NamlIZ+2vIAAAAI;
						x=12000; y=4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=PG
							role=terminal
						}
					}
					ha:group.7 {
						uuid=aXxUDjrO1NamlIZ+2vIAAAEi; src_uuid=aXxUDjrO1NamlIZ+2vIAAAAJ;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=GND
							role=terminal
						}
					}
					ha:polygon.8 {
						li:outline {
							ha:line { x1=-8000; y1=16000; x2=-8000; y2=0; }
							ha:line { x1=-8000; y1=0; x2=8000; y2=0; }
							ha:line { x1=8000; y1=0; x2=8000; y2=16000; }
							ha:line { x1=8000; y1=16000; x2=-8000; y2=16000; }
						}
						stroke=sym-decor;
						fill=sym-decor-fill;
					}
					ha:text.9 { x1=-7000; y1=10000; dyntext=0; stroke=sym-decor; text=VIN; }
					ha:text.10 { x1=-7000; y1=6000; dyntext=0; stroke=sym-decor; text=EN; }
					ha:text.11 { x1=-7000; y1=2000; dyntext=0; stroke=sym-decor; text=SS; }
					ha:text.12 { x1=7000; y1=10000; mirx=1; dyntext=0; stroke=sym-decor; text=VOUT; }
					ha:text.13 { x1=7000; y1=6000; mirx=1; dyntext=0; stroke=sym-decor; text=FB; }
					ha:text.14 { x1=7000; y1=2000; mirx=1; dyntext=0; stroke=sym-decor; text=PG; }
					ha:text.15 { x1=0; y1=16000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:text.16 { x1=-16000; y1=-4000; dyntext=1; stroke=sym-secondary; text=%../a.device%; floater=1; }
				}
				ha:attrib {
					device=TPS82130SIL
					footprint=TPS82130SILT
					name=U2
					li:portmap {
						{VIN->pcb/pinnum=2}
						{EN->pcb/pinnum=1}
						{SS->pcb/pinnum=8}
						{VOUT->pcb/pinnum=4 5}
						{FB->pcb/pinnum=6}
						{PG->pcb/pinnum=7}
						{GND->pcb/pinnum=3 EP}
					}
					role=symbol
				}
			}
			ha:connection.191 {
				li:conn {
					/2/190/1/1
					/2/188/13
				}
			}
			ha:connection.192 {
				li:conn {
					/2/190/2/1
					/2/188/10
				}
			}
			ha:connection.193 {
				li:conn {
					/2/190/3/1
					/2/35/3
				}
			}
			ha:connection.194 {
				li:conn {
					/2/190/4/1
					/2/185/8
				}
			}
			ha:connection.195 {
				li:conn {
					/2/190/5/1
					/2/68/5
				}
			}
			ha:connection.196 {
				li:conn {
					/2/190/7/1
					/2/26/1
				}
			}
			ha:connection.197 {
				li:conn {
					/2/51/2/1
					/2/188/4
				}
			}
			ha:connection.198 {
				li:conn {
					/2/51/3/1
					/2/53/1
				}
			}
		}
		ha:attrib {
			drawing_min_height=200000
			drawing_min_width=287000
			maintainer=<maint. attr>
			page=<page attr>
			print_page=A/4
			title=<please set sheet title attribute>
		}
	}
  li:sch-rnd-conf-v1 {
   ha:overwrite {
    ha:editor {
     grids_idx = 3
     grid = 4.0960 mm
    }
   }
  }
}
