## Reusable TC Route Params

----

create_user

{
    "username": "jim_the_og",
    "password": "password"
}

create_provider

{
  "provider_name": "Terry Bolea's Office",
  "patient_phone": "555-555-5555",
  "patient_zip": "88888",
}

create_eligible_case

{
  "provider_id": 1,
  "patient_id": 1,
  "insurer_id": 1,
  "iro_id": 1,
  "denial_reason": "UGH",
  "expedited": 1
}