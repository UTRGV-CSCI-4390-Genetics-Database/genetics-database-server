DROP TABLE IF EXISTS individuals, projects, project_enrollments, demographics, biological_measurements,
    psychiatric_disorders, medical_history, blood_samples, markers, categories, category_individuals, category_markers;

CREATE TABLE individuals
(
    subject_id      integer PRIMARY KEY,
    pedigree_number integer,
    name            text,
    gender          text,
    is_genotyped    boolean,
    father_id       integer,
    mother_id       integer,
    sex             char(1) CHECK ( sex = 'M' OR sex = 'F' )
);

CREATE TABLE projects
(
    id   integer PRIMARY KEY,
    name text NOT NULL
);

CREATE TABLE project_enrollments
(
    project_id    integer REFERENCES projects,
    individual_id integer REFERENCES individuals,
    PRIMARY KEY (project_id, individual_id)
);

CREATE TABLE demographics
(
    subject_id                integer PRIMARY KEY REFERENCES individuals,
    age                       smallint CHECK ( age >= 0 ),
    date_of_birth             date,
    approximate_income        smallint CHECK ( approximate_income BETWEEN 1 AND 8 ),
    country                   text,
    ethnic_category           smallint CHECK ( ethnic_category BETWEEN 1 AND 2 ),
    father_ethnicity_1        smallint CHECK ( father_ethnicity_1 BETWEEN 210 AND 290 ),
    father_ethnicity_2        smallint CHECK ( father_ethnicity_2 BETWEEN 210 AND 290 ),
    father_ethnicity_3        smallint CHECK ( father_ethnicity_3 BETWEEN 210 AND 290 ),
    father_ethnicity_4        smallint CHECK ( father_ethnicity_4 BETWEEN 210 AND 290 ),
    income_and_occupation     text,
    marital_status            smallint CHECK ( marital_status BETWEEN 1 AND 5 ),
    military_discharge        smallint CHECK ( military_discharge BETWEEN 1 AND 7 ),
    military_rejected         smallint CHECK ( military_rejected BETWEEN 1 AND 6 ),
    mother_ethnicity_1        smallint CHECK ( mother_ethnicity_1 BETWEEN 210 AND 290 ),
    mother_ethnicity_2        smallint CHECK ( mother_ethnicity_2 BETWEEN 210 AND 290 ),
    mother_ethnicity_3        smallint CHECK ( mother_ethnicity_3 BETWEEN 210 AND 290 ),
    mother_ethnicity_4        smallint CHECK ( mother_ethnicity_4 BETWEEN 210 AND 290 ),
    num_children              smallint CHECK ( num_children >= 0 ),
    num_packs_smoking_per_day real CHECK ( num_packs_smoking_per_day >= 0.0 ),
    num_pregnancies           smallint CHECK ( num_pregnancies >= 0 ),
    years_smoking             real CHECK ( years_smoking >= 0.0 ),
    occupational_disability   boolean,
    religion                  smallint CHECK ( religion BETWEEN 1 AND 6 ),
    other_religion            text CHECK ( (other_religion IS NULL) OR (religion = 6) ),
    reported_race             smallint CHECK ( reported_race BETWEEN 1 AND 6 ),
    smoked_cigarettes         smallint CHECK ( smoked_cigarettes BETWEEN 0 AND 2 ),
    times_married             smallint CHECK ( times_married >= 0 ),
    years_of_school           real CHECK ( years_of_school >= 0.0 )
);

CREATE TABLE biological_measurements
(
    id                   serial PRIMARY KEY,
    subject_id           integer REFERENCES individuals,
    bmi                  real CHECK ( bmi > 0.0 ),
    measurements_date    date,
    abdominal_girth_cm   real CHECK ( abdominal_girth_cm > 0.0 ),
    arm_circumference_cm real CHECK ( arm_circumference_cm > 0.0 ),
    height_cm            real CHECK ( height_cm > 0.0 ),
    hips_cm              real CHECK ( hips_cm > 0.0 ),
    pulse                smallint CHECK ( pulse >= 0 ),
    diastolic            smallint CHECK ( diastolic >= 0 ),
    systolic             smallint CHECK ( systolic >= 0 ),
    waist_cm             real CHECK ( waist_cm > 0.0 ),
    weight_kg            real CHECK ( weight_kg > 0.0 )
);

CREATE TABLE psychiatric_disorders
(
    subject_id                           integer PRIMARY KEY REFERENCES individuals,
    has_agoraphobia                      boolean,
    has_antisocial_disorder              boolean,
    has_anxiety_disorder                 boolean,
    has_bipolar_disorder                 boolean,
    has_borderline_intellectual_function boolean,
    has_borderline_personality_disorder  boolean,
    has_catatonia                        boolean,
    has_cyclothymia                      boolean,
    has_dissociative_disorder            boolean,
    has_dysthymia                        boolean,
    has_major_depression                 boolean,
    has_mania                            boolean,
    has_mood_disorder                    boolean,
    narrow                               smallint,
    has_panic_disorder                   boolean,
    has_psychosis                        boolean,
    has_sabp_dx                          boolean,
    has_sad_dx                           boolean,
    is_sc_catatonic                      boolean,
    is_sc_paranoid                       boolean,
    has_sc_undifferential                boolean,
    is_sc_psy_chart_reviewed             boolean,
    schizophrenia                        smallint CHECK ( schizophrenia <= 5 ),
    unspecified_mental_illness           text
);

CREATE TABLE medical_history
(
    subject_id                                   integer PRIMARY KEY REFERENCES individuals,
    abortion                                     boolean,
    alcohol_abuse_or_dependence                  boolean,
    acne                                         boolean,
    addisons_disease                             boolean,
    adrenal_problems                             boolean,
    alcohol_and_drugs_details                    text,
    alcoholism                                   boolean,
    allergies                                    boolean,
    allergies_details                            text,
    allergy_types                                text,
    alzheimers                                   boolean,
    anemia                                       boolean,
    anxiety_disorder                             boolean,
    arrhythmias                                  boolean,
    asthma                                       boolean,
    autism                                       boolean,
    bed_wetting                                  boolean,
    bipolar_disorder                             boolean,
    bipolar_type_1                               boolean,
    bipolar_type_2                               boolean,
    birth_defects                                boolean,
    bladder_cancer                               boolean,
    blood_details                                text,
    blood_problem_type                           text,
    blood_problems                               boolean,
    borderline_intellectual_function             boolean,
    bowel_movements_in_bed_or_soiling_clothes    boolean,
    brain_cancer                                 boolean,
    breast_cancer                                boolean,
    breathing_problems                           boolean,
    cancer                                       boolean,
    cancer_type                                  text,
    cancers_details                              text,
    chemical_allergies                           boolean,
    chemical_allergies_specified                 text,
    childhood_problems                           boolean,
    cocaine_abuse_or_dependence                  boolean,
    colon_cancer                                 boolean,
    congenital_heart_disease                     boolean,
    copd                                         boolean,
    coronary_artery_disease                      boolean,
    cushings_disease                             boolean,
    congenital_deafness                          boolean,
    delayed_crawling                             boolean,
    delayed_speech                               boolean,
    delayed_walking                              boolean,
    dementia                                     boolean,
    depression                                   boolean,
    developmental_and_birth_defect_type_or_other text,
    development_and_birth_defects_details        text,
    diabetes_type_1                              boolean,
    diabetes_type_2                              boolean,
    digestive_problem_type                       text,
    digestive_problems                           boolean,
    digestive_system_details                     text,
    drug_problems                                boolean,
    dyslexia                                     boolean,
    endocrine_problem_type                       text,
    endocrine_details                            text,
    epilepsy                                     boolean,
    eyesight_problems                            boolean,
    face_deformities                             boolean,
    fibromyalgia                                 boolean,
    fingers_or_toes_deformities                  boolean,
    food_allergies                               boolean,
    food_allergies_specified                     text,
    gall_bladder_problems                        boolean,
    genitourinary_system_details                 text,
    genitourinary_system_problem_or_other        text,
    glands_or_hormones_problems                  boolean,
    headaches                                    boolean,
    hearing_problems                             boolean,
    heart_diseases                               boolean,
    heart_diseases_details                       text,
    heart_problem_type                           text,
    hepatic_cancer                               boolean,
    hepatitis                                    text,
    heroin_abuse_or_dependence                   boolean,
    high_blood_pressure                          boolean,
    high_cholesterol_or_triglycerides            boolean,
    hypercholesterolemia                         boolean,
    hypertriglyceridemia                         boolean,
    history_of_malnutrition                      boolean,
    huntingtons_disease                          boolean,
    hyperparathyroidism                          boolean,
    hyperthyroidism                              boolean,
    hypoparathyroidism                           boolean,
    hypothyroidism                               boolean,
    joint_problems_type                          text,
    joint_problems                               boolean,
    joints_details                               text,
    kidney_problems                              boolean,
    learning_disabilities                        boolean,
    learning_problems                            boolean,
    leukemia                                     boolean,
    liver_problems                               boolean,
    lung_cancer                                  boolean,
    lupus                                        boolean,
    lymphoma                                     boolean,
    marijuana_abuse_or_dependence                boolean,
    medicine_allergies                           boolean,
    medicine_allergies_specified                 text,
    memory_problems                              boolean,
    meningitis                                   boolean,
    mental_retardation                           boolean,
    migraine_headaches                           boolean,
    miscarriage                                  boolean,
    movement_disorder                            boolean,
    multiple_sclerosis                           boolean,
    muscular_dystrophy                           boolean,
    myasthenia_gravis_disease                    boolean,
    myocardial_infarction                        boolean,
    neurological_details                         text,
    neurological_diseases                        boolean,
    neurological_problem_type                    text,
    no_medical_history_as_per_informant          boolean,
    osteoarthritis                               boolean,
    other_alcohol_or_drug_problems_or_disorders  text,
    other_allergy_problems                       text,
    pacemaker                                    boolean,
    paralysis                                    boolean,
    parathyroid_problems                         boolean,
    parkinsons_disease                           boolean,
    peptic_ulcer_disease                         boolean,
    pregnancy_complications                      boolean,
    pregnancy_complications_type                 text,
    prostate_cancer                              boolean,
    prostate_problems                            boolean,
    psoriasis                                    boolean,
    psychiatric_details                          text,
    psychiatric_disorders_or_other               text,
    psychiatric_or_psychological_problem         boolean,
    ptsd                                         boolean,
    rash                                         boolean,
    respiratory_system_details                   text,
    respiratory_problem_type                     text,
    rheumatic_fever                              boolean,
    rheumatoid_arthritis                         boolean,
    schizoaffective_disorder                     boolean,
    seasonal_allergies                           boolean,
    seasonal_allergies_specified                 text,
    seeing_or_hearing_problems                   boolean,
    seizures                                     boolean,
    sensory_organ_details                        text,
    sensory_problem_type                         text,
    sjoegren_syndrome                            boolean,
    skin_cancer                                  boolean,
    skin_details                                 text,
    skin_problem_type                            text,
    skin_problems                                boolean,
    solvents_abuse_or_dependence                 boolean,
    solvents_abuse_or_dependence_type            boolean,
    stillbirth                                   boolean,
    stroke                                       boolean,
    sugar_problems                               boolean,
    suspected_mh_issues                          text,
    teeth_deformities                            boolean,
    thymoma                                      boolean,
    thyroid_problems                             boolean,
    tourettes_syndrome                           boolean,
    ulcerative_colitis                           boolean
);

CREATE TABLE blood_samples
(
    id         text PRIMARY KEY,
    subject_id integer REFERENCES individuals
);

CREATE TABLE markers
(
    chromosome_name text,
    marker_name     text PRIMARY KEY,
    base_pairs      bigint,
    allele_a_value  char(1),
    allele_b_value  char(1),
    minor_allele    char(1),
    ancestry        boolean,
    diabetes        boolean,
    afr_a1          real,
    afr_a2          real,
    eas_a1          real,
    eas_a2          real,
    eur_a1          real,
    eur_a2          real,
    gene            text,
    marker_set      text,
    natam_a1        real,
    natam_a2        real,
    reference       text
);

CREATE TABLE categories
(
    id          serial PRIMARY KEY,
    name        text NOT NULL,
    description text
);

CREATE TABLE category_individuals
(
    category_id integer REFERENCES categories,
    subject_id  integer REFERENCES individuals,
    PRIMARY KEY (category_id, subject_id)
);

CREATE TABLE category_markers
(
    category_id integer REFERENCES categories,
    marker_name text REFERENCES markers,
    PRIMARY KEY (category_id, marker_name)
);

GRANT SELECT, INSERT, UPDATE, DELETE, TRUNCATE
    ON individuals, projects, project_enrollments, demographics, biological_measurements, psychiatric_disorders,
    medical_history, blood_samples, markers, categories, category_individuals, category_markers
    TO genetics_user;
