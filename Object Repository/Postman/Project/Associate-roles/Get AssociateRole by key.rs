<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get AssociateRole by key</name>
   <tag></tag>
   <elementGuidId>fa1b761e-1a00-413c-b9f2-a630330c7750</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer {{ctp_access_token}}</value>
      <webElementGuid>248d24a7-d42a-4cfa-8ba0-740b682f75ec</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>67cc51fe-190c-4e03-80ab-a3d59056bdd5</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${host}/${project-key}/associate-roles/key=${associate-role-key}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>030179b1-3f3d-4e33-b8fa-6f7151aab88e</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>0ad93104-d5b9-4ae1-b3be-d70504fe0576</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.associate-role-key</defaultValue>
      <description></description>
      <id>d009ec85-9356-438b-a2e3-96b5c9b76a29</id>
      <masked>false</masked>
      <name>associate-role-key</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
